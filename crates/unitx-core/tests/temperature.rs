use unitx_core::temperature::{convert, TemperatureUnit::*};

#[test]
fn c_to_f_roundtrip() {
    let c = 37.0;
    let f = convert(c, C, F);
    let back = convert(f, F, C);
    assert!((back - c).abs() < 1e-9);
}

#[test]
fn c_to_k_zero() {
    let k = convert(0.0, C, K);
    assert!((k - 273.15).abs() < 1e-9);
}
