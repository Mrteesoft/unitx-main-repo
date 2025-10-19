#[derive(Clone, Copy, Debug)]
pub enum DistanceUnit { M, KM, MI }

impl DistanceUnit {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_uppercase().as_str() {
            "M"  => Some(Self::M),
            "KM" => Some(Self::KM),
            "MI" => Some(Self::MI),
            _ => None,
        }
    }
}

// constants (exact where possible)
const M_PER_KM: f64 = 1000.0;
const M_PER_MI: f64 = 1609.344; // exact statute mile in meters

pub fn convert(value: f64, from: DistanceUnit, to: DistanceUnit) -> f64 {
    // normalize to meters
    let meters = match from {
        DistanceUnit::M  => value,
        DistanceUnit::KM => value * M_PER_KM,
        DistanceUnit::MI => value * M_PER_MI,
    };
    // meters -> target
    match to {
        DistanceUnit::M  => meters,
        DistanceUnit::KM => meters / M_PER_KM,
        DistanceUnit::MI => meters / M_PER_MI,
    }
}