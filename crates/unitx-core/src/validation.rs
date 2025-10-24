use crate::error::UnitxError;

pub fn validate_temperature_value(value: f64) -> Result<(), UnitxError> {
    if !value.is_finite() {
        return Err(UnitxError::ValidationError("Temperature must be finite".to_string()));
    }
    if value < -273.15 {
        return Err(UnitxError::ValidationError("Temperature below absolute zero (-273.15°C)".to_string()));
    }
    if value > 1e6 {
        return Err(UnitxError::ValidationError("Temperature too high (max 1,000,000)".to_string()));
    }
    Ok(())
}

pub fn validate_distance_value(value: f64) -> Result<(), UnitxError> {
    if !value.is_finite() {
        return Err(UnitxError::ValidationError("Distance must be finite".to_string()));
    }
    if value < 0.0 {
        return Err(UnitxError::ValidationError("Distance cannot be negative".to_string()));
    }
    if value > 1e12 {
        return Err(UnitxError::ValidationError("Distance too large (max 1 trillion)".to_string()));
    }
    Ok(())
}

pub fn validate_currency_value(value: &str) -> Result<(), UnitxError> {
    use rust_decimal::Decimal;
    use std::str::FromStr;
    
    if value.is_empty() {
        return Err(UnitxError::ValidationError("Currency amount cannot be empty".to_string()));
    }
    
    if value.len() > 20 {
        return Err(UnitxError::ValidationError("Currency amount too long (max 20 chars)".to_string()));
    }
    
    match Decimal::from_str(value) {
        Ok(decimal) => {
            if decimal < Decimal::ZERO {
                Err(UnitxError::ValidationError("Currency amount cannot be negative".to_string()))
            } else if decimal > Decimal::from(1_000_000_000i64) {
                Err(UnitxError::ValidationError("Currency amount too large (max 1 billion)".to_string()))
            } else {
                Ok(())
            }
        }
        Err(_) => Err(UnitxError::ValidationError("Invalid currency amount format".to_string())),
    }
}

pub fn validate_unit_string(unit: &str, category: &str) -> Result<(), UnitxError> {
    if unit.is_empty() {
        return Err(UnitxError::ValidationError(format!("{} unit cannot be empty", category)));
    }
    if unit.len() > 10 {
        return Err(UnitxError::ValidationError(format!("{} unit too long (max 10 chars)", category)));
    }
    if !unit.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(UnitxError::ValidationError(format!("{} unit must contain only letters", category)));
    }
    Ok(())
}