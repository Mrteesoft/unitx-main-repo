use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum CurrencyUnit { USD, EUR, GBP, JPY }

impl CurrencyUnit {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_uppercase().as_str() {
            "USD" => Some(Self::USD),
            "EUR" => Some(Self::EUR),
            "GBP" => Some(Self::GBP),
            "JPY" => Some(Self::JPY),
            _ => None,
        }
    }
}

// Mock exchange rates (USD base)
fn get_usd_rate(currency: CurrencyUnit) -> Decimal {
    match currency {
        CurrencyUnit::USD => Decimal::from_str("1.0").unwrap(),
        CurrencyUnit::EUR => Decimal::from_str("0.85").unwrap(),
        CurrencyUnit::GBP => Decimal::from_str("0.75").unwrap(),
        CurrencyUnit::JPY => Decimal::from_str("150.0").unwrap(),
    }
}

pub fn convert(value: Decimal, from: CurrencyUnit, to: CurrencyUnit) -> Decimal {
    // normalize to USD
    let usd = value / get_usd_rate(from);
    // USD -> target
    usd * get_usd_rate(to)
}