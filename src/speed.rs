use std::fmt;
use std::str::FromStr;
use crate::errors::{AppError, suggest_unit};
use crate::unit::LinearUnit;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpeedUnit { Kmh, Ms, Mph }

impl FromStr for SpeedUnit {
    type Err = AppError;
    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        match unit.to_lowercase().as_str() {
            "kmh" | "km/h" => Ok(SpeedUnit::Kmh), "ms" | "m/s" => Ok(SpeedUnit::Ms), "mph" => Ok(SpeedUnit::Mph),
            _ => { let suggestion = suggest_unit(unit, Self::valid_str_units()); match suggestion { Some(s) => Err(AppError::InvalidUnitWithSuggestion(unit.to_string(), s)), None => Err(AppError::InvalidUnit(unit.to_string())) } }
        }
    }
}

impl fmt::Display for SpeedUnit { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { match self { SpeedUnit::Kmh => write!(f, "km/h"), SpeedUnit::Ms => write!(f, "m/s"), SpeedUnit::Mph => write!(f, "mph") } } }

impl LinearUnit for SpeedUnit {
    fn to_base(&self, value: f64) -> f64 { match self { SpeedUnit::Ms => value, SpeedUnit::Kmh => value / 3.6, SpeedUnit::Mph => value * 0.44704 } }
    fn convert_from_base(&self, value: f64) -> f64 { match self { SpeedUnit::Ms => value, SpeedUnit::Kmh => value * 3.6, SpeedUnit::Mph => value / 0.44704 } }
    fn variants() -> &'static [Self] { &[SpeedUnit::Kmh, SpeedUnit::Ms, SpeedUnit::Mph] }
    fn valid_str_units() -> &'static [&'static str] { &["kmh", "km/h", "ms", "m/s", "mph"] }
}