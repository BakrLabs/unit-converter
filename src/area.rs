use std::fmt;
use std::str::FromStr;
use crate::errors::{AppError, suggest_unit};
use crate::unit::LinearUnit;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AreaUnit { Sqm, Sqkm, Acre }

impl FromStr for AreaUnit {
    type Err = AppError;
    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        match unit.to_lowercase().as_str() {
            "sqm" | "m2" => Ok(AreaUnit::Sqm), "sqkm" | "km2" => Ok(AreaUnit::Sqkm), "acre" => Ok(AreaUnit::Acre),
            _ => { let suggestion = suggest_unit(unit, Self::valid_str_units()); match suggestion { Some(s) => Err(AppError::InvalidUnitWithSuggestion(unit.to_string(), s)), None => Err(AppError::InvalidUnit(unit.to_string())) } }
        }
    }
}

impl fmt::Display for AreaUnit { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { match self { AreaUnit::Sqm => write!(f, "m²"), AreaUnit::Sqkm => write!(f, "km²"), AreaUnit::Acre => write!(f, "acre") } } }

impl LinearUnit for AreaUnit {
    fn to_base(&self, value: f64) -> f64 { match self { AreaUnit::Sqm => value, AreaUnit::Sqkm => value * 1_000_000.0, AreaUnit::Acre => value * 4046.86 } }
    fn convert_from_base(&self, value: f64) -> f64 { match self { AreaUnit::Sqm => value, AreaUnit::Sqkm => value / 1_000_000.0, AreaUnit::Acre => value / 4046.86 } }
    fn variants() -> &'static [Self] { &[AreaUnit::Sqm, AreaUnit::Sqkm, AreaUnit::Acre] }
    fn valid_str_units() -> &'static [&'static str] { &["sqm", "m2", "sqkm", "km2", "acre"] }
}