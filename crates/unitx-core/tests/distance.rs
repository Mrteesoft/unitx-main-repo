use unitx_core::distance::{convert, DistanceUnit::*};

#[test]
fn km_to_m_roundtrip() {
    let km = 3.5;
    let m = convert(km, KM, M);
    let back = convert(m, M, KM);
    assert!((back - km).abs() < 1e-12);
}

#[test]
fn mi_to_km_known_value() {
    // 1 mile = 1.609344 km (exact)
    let km = convert(1.0, MI, KM);
    assert!((km - 1.609_344).abs() < 1e-12);
}
