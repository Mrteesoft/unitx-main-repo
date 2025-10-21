use rust_decimal::Decimal;
use crate::providers::ExchangeRateProvider;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

pub fn convert_with_provider<P: ExchangeRateProvider>(
    value: Decimal,
    from: CurrencyUnit,
    to: CurrencyUnit,
    provider: &P,
) -> Result<Decimal, String> {
    let rate = provider.get_rate(from, to)?;
    Ok(value * rate)
}

// Legacy function for backward compatibility
pub fn convert(value: Decimal, from: CurrencyUnit, to: CurrencyUnit) -> Decimal {
    use crate::providers::MockProvider;
    let provider = MockProvider;
    convert_with_provider(value, from, to, &provider).unwrap_or(Decimal::ZERO)
}