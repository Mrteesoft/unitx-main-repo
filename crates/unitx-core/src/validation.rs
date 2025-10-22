use crate::error::UnitxError;

pub fn validate_temperature_value(value: f64) -> Result<(), UnitxError> {
    if value < -273.15 {
        return Err(UnitxError::InvalidUnit("Temperature below absolute zero".to_string()));
    }
    if !value.is_finite() {
        return Err(UnitxError::InvalidUnit("Temperature must be finite".to_string()));
    }
    Ok(())
}

pub fn validate_distance_value(value: f64) -> Result<(), UnitxError> {
    if value < 0.0 {
        return Err(UnitxError::InvalidUnit("Distance cannot be negative".to_string()));
    }
    if !value.is_finite() {
        return Err(UnitxError::InvalidUnit("Distance must be finite".to_string()));
    }
    Ok(())
}

pub fn validate_currency_value(value: &str) -> Result<(), UnitxError> {
    use rust_decimal::Decimal;
    use std::str::FromStr;
    
    match Decimal::from_str(value) {
        Ok(decimal) => {
            if decimal < Decimal::ZERO {
                Err(UnitxError::InvalidUnit("Currency amount cannot be negative".to_string()))
            } else {
                Ok(())
            }
        }
        Err(_) => Err(UnitxError::InvalidUnit("Invalid currency amount format".to_string())),
    }
}