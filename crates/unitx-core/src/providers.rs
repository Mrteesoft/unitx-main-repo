use rust_decimal::Decimal;
use crate::currency::CurrencyUnit;
use std::str::FromStr;

pub trait ExchangeRateProvider {
    fn get_rate(&self, from: CurrencyUnit, to: CurrencyUnit) -> Result<Decimal, String>;
}

pub struct MockProvider;

impl ExchangeRateProvider for MockProvider {
    fn get_rate(&self, from: CurrencyUnit, to: CurrencyUnit) -> Result<Decimal, String> {
        let usd_rates = match from {
            CurrencyUnit::USD => Decimal::from_str("1.0").unwrap(),
            CurrencyUnit::EUR => Decimal::from_str("0.85").unwrap(),
            CurrencyUnit::GBP => Decimal::from_str("0.75").unwrap(),
            CurrencyUnit::JPY => Decimal::from_str("150.0").unwrap(),
        };
        
        let to_rates = match to {
            CurrencyUnit::USD => Decimal::from_str("1.0").unwrap(),
            CurrencyUnit::EUR => Decimal::from_str("0.85").unwrap(),
            CurrencyUnit::GBP => Decimal::from_str("0.75").unwrap(),
            CurrencyUnit::JPY => Decimal::from_str("150.0").unwrap(),
        };
        
        Ok(to_rates / usd_rates)
    }
}

pub struct FixedRateProvider {
    rates: std::collections::HashMap<(CurrencyUnit, CurrencyUnit), Decimal>,
}

impl FixedRateProvider {
    pub fn new() -> Self {
        let mut rates = std::collections::HashMap::new();
        
        // USD base rates
        rates.insert((CurrencyUnit::USD, CurrencyUnit::EUR), Decimal::from_str("0.85").unwrap());
        rates.insert((CurrencyUnit::USD, CurrencyUnit::GBP), Decimal::from_str("0.75").unwrap());
        rates.insert((CurrencyUnit::USD, CurrencyUnit::JPY), Decimal::from_str("150.0").unwrap());
        
        // Reverse rates
        rates.insert((CurrencyUnit::EUR, CurrencyUnit::USD), Decimal::from_str("1.176470588").unwrap());
        rates.insert((CurrencyUnit::GBP, CurrencyUnit::USD), Decimal::from_str("1.333333333").unwrap());
        rates.insert((CurrencyUnit::JPY, CurrencyUnit::USD), Decimal::from_str("0.006666667").unwrap());
        
        Self { rates }
    }
}

impl ExchangeRateProvider for FixedRateProvider {
    fn get_rate(&self, from: CurrencyUnit, to: CurrencyUnit) -> Result<Decimal, String> {
        if from == to {
            return Ok(Decimal::from_str("1.0").unwrap());
        }
        
        self.rates.get(&(from, to))
            .copied()
            .ok_or_else(|| format!("No rate available for {:?} to {:?}", from, to))
    }
}