# unitx-core

Core library for the unitx project. Provides temperature, distance, and live currency conversions with validation utilities.

## Usage

```bash
cargo add unitx-core
```

```rust
use rust_decimal::Decimal;
use unitx_core::currency::{convert, CurrencyUnit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let amount = Decimal::new(10000, 2); // 100.00
    let eur = convert(amount, CurrencyUnit::USD, CurrencyUnit::EUR)?;
    println!("USD {} -> EUR {}", amount, eur);
    Ok(())
}
```

Currency rates are sourced from the European Central Bank daily feed (no API keys required).

Live FX roundtrip tests are marked `#[ignore]`; run `cargo test -- --ignored` with network access to exercise them.
