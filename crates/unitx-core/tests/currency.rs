use unitx_core::currency::{convert, CurrencyUnit::*};
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn usd_to_eur_roundtrip() {
    let usd = Decimal::from_str("100.0").unwrap();
    let eur = convert(usd, USD, EUR);
    let back = convert(eur, EUR, USD);
    assert!((back - usd).abs() < Decimal::from_str("0.01").unwrap());
}

#[test]
fn same_currency_no_change() {
    let amount = Decimal::from_str("50.0").unwrap();
    let result = convert(amount, USD, USD);
    assert_eq!(result, amount);
}