use rust_decimal::Decimal;
use std::error::Error;
use std::str::FromStr;
use unitx_core::currency::{convert, CurrencyUnit};

fn main() -> Result<(), Box<dyn Error>> {
    let amount = Decimal::from_str("100.00")?;
    let converted = convert(amount, CurrencyUnit::USD, CurrencyUnit::EUR)?;

    println!("USD {} -> EUR {}", amount, converted);
    Ok(())
}
