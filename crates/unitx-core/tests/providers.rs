use unitx_core::currency::{convert_with_provider, CurrencyUnit::*};
use unitx_core::providers::{MockProvider, FixedRateProvider};
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn mock_provider_usd_to_eur() {
    let provider = MockProvider;
    let amount = Decimal::from_str("100.0").unwrap();
    let result = convert_with_provider(amount, USD, EUR, &provider).unwrap();
    assert_eq!(result, Decimal::from_str("85.0").unwrap());
}

#[test]
fn fixed_rate_provider_same_currency() {
    let provider = FixedRateProvider::new();
    let amount = Decimal::from_str("50.0").unwrap();
    let result = convert_with_provider(amount, USD, USD, &provider).unwrap();
    assert_eq!(result, amount);
}

#[test]
fn fixed_rate_provider_usd_to_eur() {
    let provider = FixedRateProvider::new();
    let amount = Decimal::from_str("100.0").unwrap();
    let result = convert_with_provider(amount, USD, EUR, &provider).unwrap();
    assert_eq!(result, Decimal::from_str("85.0").unwrap());
}