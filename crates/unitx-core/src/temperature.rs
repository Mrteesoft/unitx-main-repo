#[derive(Clone, Copy, Debug)]
pub enum TemperatureUnit {
    C,
    F,
    K,
}

impl TemperatureUnit {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_uppercase().as_str() {
            "C" => Some(Self::C),
            "F" => Some(Self::F),
            "K" => Some(Self::K),
            _ => None,
        }
    }
}

pub fn convert(value: f64, from: TemperatureUnit, to: TemperatureUnit) -> f64 {
    // normalize to Kelvin
    let k = match from {
        TemperatureUnit::C => value + 273.15,
        TemperatureUnit::F => (value - 32.0) * 5.0 / 9.0 + 273.15,
        TemperatureUnit::K => value,
    };
    // Kelvin -> target
    match to {
        TemperatureUnit::C => k - 273.15,
        TemperatureUnit::F => (k - 273.15) * 9.0 / 5.0 + 32.0,
        TemperatureUnit::K => k,
    }
}
