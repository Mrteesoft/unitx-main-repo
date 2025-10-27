use unitx_core::validation::*;

#[test]
fn temperature_validation_edge_cases() {
    assert!(validate_temperature_value(-273.15).is_ok());
    assert!(validate_temperature_value(-273.16).is_err());
    assert!(validate_temperature_value(1_000_000.0).is_ok());
    assert!(validate_temperature_value(1_000_001.0).is_err());
}

#[test]
fn distance_validation_edge_cases() {
    assert!(validate_distance_value(0.0).is_ok());
    assert!(validate_distance_value(1e12).is_ok());
    assert!(validate_distance_value(1e13).is_err());
}

#[test]
fn currency_validation_edge_cases() {
    assert!(validate_currency_value("0").is_ok());
    assert!(validate_currency_value("1000000000").is_ok());
    assert!(validate_currency_value("1000000001").is_err());
    assert!(validate_currency_value("").is_err());
    assert!(validate_currency_value("123456789012345678901").is_err()); // too long
}

#[test]
fn unit_string_validation() {
    assert!(validate_unit_string("USD", "Currency").is_ok());
    assert!(validate_unit_string("C", "Temperature").is_ok());
    assert!(validate_unit_string("", "Currency").is_err());
    assert!(validate_unit_string("TOOLONGUNIT", "Currency").is_err());
    assert!(validate_unit_string("US1", "Currency").is_err()); // contains number
    assert!(validate_unit_string("US-D", "Currency").is_err()); // contains hyphen
}
