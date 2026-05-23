use std::fmt;
use std::str::FromStr;
use crate::errors::{AppError, suggest_unit};
use crate::unit::LinearUnit;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MassUnit { Mg, G, Kg }

impl FromStr for MassUnit {
    type Err = AppError;
    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        match unit.to_lowercase().as_str() {
            "mg" | "milligram" | "milligrams" => Ok(MassUnit::Mg),
"g" | "gram" | "grams" => Ok(MassUnit::G),
"kg" | "kilogram" | "kilograms" => Ok(MassUnit::Kg),
            _ => { let suggestion = suggest_unit(unit, Self::valid_str_units()); match suggestion { Some(s) => Err(AppError::InvalidUnitWithSuggestion(unit.to_string(), s)), None => Err(AppError::InvalidUnit(unit.to_string())) } }
        }
    }
}

impl fmt::Display for MassUnit { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { match self { MassUnit::Mg => write!(f, "mg"), MassUnit::G => write!(f, "g"), MassUnit::Kg => write!(f, "kg") } } }

impl LinearUnit for MassUnit {
    fn to_base(&self, value: f64) -> f64 { match self { MassUnit::Mg => value / 1000.0, MassUnit::G => value, MassUnit::Kg => value * 1000.0 } }
    fn convert_from_base(&self, value: f64) -> f64 { match self { MassUnit::Mg => value * 1000.0, MassUnit::G => value, MassUnit::Kg => value / 1000.0 } }
    fn variants() -> &'static [Self] { &[MassUnit::Mg, MassUnit::G, MassUnit::Kg] }
    fn valid_str_units() -> &'static [&'static str] { &["mg", "milligram", "milligrams", "g", "gram", "grams", "kg", "kilogram", "kilograms"] }
}