use crate::currency::CurrencyUnit;
use crate::providers::ExchangeRateProvider;
use isahc::{prelude::*, HttpClient, Request};
use quick_xml::events::Event;
use quick_xml::Reader;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const DEFAULT_SOURCE: &str = "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml";

/// Provider that fetches live FX rates using the European Central Bank daily feed.
#[derive(Clone)]
pub struct LiveExchangeProvider {
    client: HttpClient,
    source_url: String,
    cache: Arc<Mutex<HashMap<(CurrencyUnit, CurrencyUnit), Decimal>>>,
    base_rates: Arc<Mutex<Option<HashMap<CurrencyUnit, Decimal>>>>,
}

impl LiveExchangeProvider {
    /// Create a provider that hits the default ECB endpoint.
    pub fn new(_unused_access_key: Option<String>) -> Self {
        Self::with_base_url(DEFAULT_SOURCE, None)
    }

    /// Create a provider pointing at a custom ECB-compatible endpoint.
    pub fn with_base_url(base_url: impl Into<String>, _unused_access_key: Option<String>) -> Self {
        let client = HttpClient::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("failed to build HTTP client");

        Self {
            client,
            source_url: base_url.into(),
            cache: Arc::new(Mutex::new(HashMap::new())),
            base_rates: Arc::new(Mutex::new(None)),
        }
    }

    fn get_base_rates(&self) -> Result<HashMap<CurrencyUnit, Decimal>, String> {
        let mut guard = self.base_rates.lock().expect("live rates cache poisoned");

        if let Some(rates) = guard.as_ref() {
            return Ok(rates.clone());
        }

        let fetched = self.fetch_base_rates()?;
        *guard = Some(fetched.clone());
        Ok(fetched)
    }

    fn fetch_base_rates(&self) -> Result<HashMap<CurrencyUnit, Decimal>, String> {
        let request = Request::get(&self.source_url)
            .timeout(Duration::from_secs(5))
            .header("Accept", "application/xml")
            .body(())
            .map_err(|err| format!("failed to build ECB request: {}", err))?;

        let mut response = self
            .client
            .send(request)
            .map_err(|err| format!("failed to fetch ECB rates: {}", err))?;

        if !response.status().is_success() {
            return Err(format!(
                "ECB rates endpoint returned HTTP {}",
                response.status()
            ));
        }

        let body = response
            .text()
            .map_err(|err| format!("failed to read ECB payload: {}", err))?;

        let mut reader = Reader::from_str(&body);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut rates = HashMap::new();
        rates.insert(CurrencyUnit::EUR, Decimal::ONE);

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e)) => {
                    if e.name().as_ref() == b"Cube" {
                        let mut currency = None;
                        let mut rate = None;

                        for attr in e.attributes() {
                            match attr {
                                Ok(attr) => match attr.key.as_ref() {
                                    b"currency" => {
                                        currency =
                                            Some(String::from_utf8_lossy(&attr.value).into_owned());
                                    }
                                    b"rate" => {
                                        rate =
                                            Some(String::from_utf8_lossy(&attr.value).into_owned());
                                    }
                                    _ => {}
                                },
                                Err(err) => {
                                    return Err(format!("invalid attribute in ECB feed: {}", err));
                                }
                            }
                        }

                        if let (Some(code), Some(rate_str)) = (currency, rate) {
                            if let Some(unit) = CurrencyUnit::parse(&code) {
                                match Decimal::from_str(&rate_str) {
                                    Ok(value) => {
                                        rates.insert(unit, value);
                                    }
                                    Err(err) => {
                                        return Err(format!(
                                            "invalid rate for {} in ECB feed: {}",
                                            code, err
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(err) => {
                    return Err(format!("failed to parse ECB feed: {}", err));
                }
                _ => {}
            }
            buf.clear();
        }

        Ok(rates)
    }
}

impl ExchangeRateProvider for LiveExchangeProvider {
    fn get_rate(&self, from: CurrencyUnit, to: CurrencyUnit) -> Result<Decimal, String> {
        if from == to {
            return Ok(Decimal::ONE);
        }

        if let Some(rate) = self
            .cache
            .lock()
            .expect("live rates cache poisoned")
            .get(&(from, to))
            .copied()
        {
            return Ok(rate);
        }

        let rates = self.get_base_rates()?;

        let from_rate = rates
            .get(&from)
            .ok_or_else(|| format!("ECB feed does not provide a rate for {:?}", from))?;

        let to_rate = rates
            .get(&to)
            .ok_or_else(|| format!("ECB feed does not provide a rate for {:?}", to))?;

        let computed = if from == CurrencyUnit::EUR {
            *to_rate
        } else if to == CurrencyUnit::EUR {
            Decimal::ONE
                .checked_div(*from_rate)
                .ok_or_else(|| format!("division overflow converting {:?} to EUR", from))?
        } else {
            to_rate
                .checked_div(*from_rate)
                .ok_or_else(|| format!("division overflow converting {:?} -> {:?}", from, to))?
        };

        self.cache
            .lock()
            .expect("live rates cache poisoned")
            .insert((from, to), computed);

        Ok(computed)
    }
}
