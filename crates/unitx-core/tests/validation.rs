use unitx_core::validation::*;

#[test]
fn temperature_validation_valid() {
    assert!(validate_temperature_value(25.0).is_ok());
    assert!(validate_temperature_value(-273.15).is_ok());
    assert!(validate_temperature_value(0.0).is_ok());
}

#[test]
fn temperature_validation_invalid() {
    assert!(validate_temperature_value(-300.0).is_err());
    assert!(validate_temperature_value(f64::NAN).is_err());
    assert!(validate_temperature_value(f64::INFINITY).is_err());
}

#[test]
fn distance_validation_valid() {
    assert!(validate_distance_value(0.0).is_ok());
    assert!(validate_distance_value(100.5).is_ok());
}

#[test]
fn distance_validation_invalid() {
    assert!(validate_distance_value(-1.0).is_err());
    assert!(validate_distance_value(f64::NAN).is_err());
}

#[test]
fn currency_validation_valid() {
    assert!(validate_currency_value("100.00").is_ok());
    assert!(validate_currency_value("0").is_ok());
    assert!(validate_currency_value("0.01").is_ok());
}

#[test]
fn currency_validation_invalid() {
    assert!(validate_currency_value("-50.00").is_err());
    assert!(validate_currency_value("invalid").is_err());
    assert!(validate_currency_value("").is_err());
}
