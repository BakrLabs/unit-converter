use std::fmt;
use std::str::FromStr;
use crate::errors::{AppError, suggest_unit};
use crate::unit::LinearUnit;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DistanceUnit { Mm, Cm, M, Km }

impl FromStr for DistanceUnit {
    type Err = AppError;
    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        match unit.to_lowercase().as_str() {
            "mm" | "millimeter" => Ok(DistanceUnit::Mm), "cm" | "centimeter" => Ok(DistanceUnit::Cm), "m" | "meter" => Ok(DistanceUnit::M), "km" | "kilometer" => Ok(DistanceUnit::Km),
            _ => { let suggestion = suggest_unit(unit, Self::valid_str_units()); match suggestion { Some(s) => Err(AppError::InvalidUnitWithSuggestion(unit.to_string(), s)), None => Err(AppError::InvalidUnit(unit.to_string())) } }
        }
    }
}

impl fmt::Display for DistanceUnit { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { match self { DistanceUnit::Mm => write!(f, "mm"), DistanceUnit::Cm => write!(f, "cm"), DistanceUnit::M => write!(f, "m"), DistanceUnit::Km => write!(f, "km") } } }

impl LinearUnit for DistanceUnit {
    fn to_base(&self, value: f64) -> f64 { match self { DistanceUnit::Mm => value / 1000.0, DistanceUnit::Cm => value / 100.0, DistanceUnit::M => value, DistanceUnit::Km => value * 1000.0 } }
    fn convert_from_base(&self, value: f64) -> f64 { match self { DistanceUnit::Mm => value * 1000.0, DistanceUnit::Cm => value * 100.0, DistanceUnit::M => value, DistanceUnit::Km => value / 1000.0 } }
    fn variants() -> &'static [Self] { &[DistanceUnit::Mm, DistanceUnit::Cm, DistanceUnit::M, DistanceUnit::Km] }
    fn valid_str_units() -> &'static [&'static str] { &["mm", "millimeter", "cm", "centimeter", "m", "meter", "km", "kilometer"] }
}