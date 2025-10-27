use rust_decimal::Decimal;
use std::str::FromStr;
use unitx_core::currency::{convert, convert_with_provider, CurrencyUnit::*};
use unitx_core::providers::LiveExchangeProvider;

fn live_provider() -> LiveExchangeProvider {
    LiveExchangeProvider::new(None)
}

#[test]
fn same_currency_no_change() {
    let amount = Decimal::from_str("50.0").unwrap();
    let result = convert(amount, USD, USD).unwrap();
    assert_eq!(result, amount);
}

#[test]
#[ignore = "requires outbound network access to the ECB feed"]
fn live_usd_to_eur_roundtrip() {
    let amount = Decimal::from_str("100.0").unwrap();
    let provider = live_provider();
    let eur = convert_with_provider(amount, USD, EUR, &provider).unwrap();
    assert!(eur > Decimal::ZERO);

    // Allow a generous delta because live rates fluctuate.
    let back = convert_with_provider(eur, EUR, USD, &provider).unwrap();
    let diff = (back - amount).abs();
    assert!(diff < Decimal::from_str("10.0").unwrap());
}
