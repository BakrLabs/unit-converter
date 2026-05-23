use std::fmt;
use std::str::FromStr;
use crate::errors::{AppError, suggest_unit};
use crate::unit::LinearUnit;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VolumeUnit { Ml, L, Gal }

impl FromStr for VolumeUnit {
    type Err = AppError;
    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        match unit.to_lowercase().as_str() {
            "ml" | "milliliter" => Ok(VolumeUnit::Ml), "l" | "liter" => Ok(VolumeUnit::L), "gal" | "gallon" => Ok(VolumeUnit::Gal),
            _ => { let suggestion = suggest_unit(unit, Self::valid_str_units()); match suggestion { Some(s) => Err(AppError::InvalidUnitWithSuggestion(unit.to_string(), s)), None => Err(AppError::InvalidUnit(unit.to_string())) } }
        }
    }
}

impl fmt::Display for VolumeUnit { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { match self { VolumeUnit::Ml => write!(f, "ml"), VolumeUnit::L => write!(f, "L"), VolumeUnit::Gal => write!(f, "gal") } } }

impl LinearUnit for VolumeUnit {
    fn to_base(&self, value: f64) -> f64 { match self { VolumeUnit::Ml => value / 1000.0, VolumeUnit::L => value, VolumeUnit::Gal => value * 3.78541 } }
    fn convert_from_base(&self, value: f64) -> f64 { match self { VolumeUnit::Ml => value * 1000.0, VolumeUnit::L => value, VolumeUnit::Gal => value / 3.78541 } }
    fn variants() -> &'static [Self] { &[VolumeUnit::Ml, VolumeUnit::L, VolumeUnit::Gal] }
    fn valid_str_units() -> &'static [&'static str] { &["ml", "milliliter", "l", "liter", "gal", "gallon"] }
}