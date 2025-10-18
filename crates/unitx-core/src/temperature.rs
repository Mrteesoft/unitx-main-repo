#[derive(Clone, Copy, Debug)]
pub enum TemperatureUnit { C, F, K }

pub fn convert(value: f64, from: TemperatureUnit, to: TemperatureUnit) -> f64 {
    let k = match from {
        TemperatureUnit::C => value + 273.15,
        TemperatureUnit::F => (value - 32.0) * 5.0/9.0 + 273.15,
        TemperatureUnit::K => value,
    };
    match to {
        TemperatureUnit::C => k - 273.15,
        TemperatureUnit::F => (k - 273.15) * 9.0/5.0 + 32.0,
        TemperatureUnit::K => k,
    }
}