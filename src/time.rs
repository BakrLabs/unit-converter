use std::fmt;
use std::str::FromStr;
use crate::errors::{AppError, suggest_unit};
use crate::unit::LinearUnit;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeUnit { S, Min, H }

impl FromStr for TimeUnit {
    type Err = AppError;
    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        match unit.to_lowercase().as_str() {
            "s" | "sec" | "second" => Ok(TimeUnit::S), "min" | "minute" => Ok(TimeUnit::Min), "h" | "hr" | "hour" => Ok(TimeUnit::H),
            _ => { let suggestion = suggest_unit(unit, Self::valid_str_units()); match suggestion { Some(s) => Err(AppError::InvalidUnitWithSuggestion(unit.to_string(), s)), None => Err(AppError::InvalidUnit(unit.to_string())) } }
        }
    }
}

impl fmt::Display for TimeUnit { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { match self { TimeUnit::S => write!(f, "s"), TimeUnit::Min => write!(f, "min"), TimeUnit::H => write!(f, "h") } } }

impl LinearUnit for TimeUnit {
    fn to_base(&self, value: f64) -> f64 { match self { TimeUnit::S => value, TimeUnit::Min => value * 60.0, TimeUnit::H => value * 3600.0 } }
    fn convert_from_base(&self, value: f64) -> f64 { match self { TimeUnit::S => value, TimeUnit::Min => value / 60.0, TimeUnit::H => value / 3600.0 } }
    fn variants() -> &'static [Self] { &[TimeUnit::S, TimeUnit::Min, TimeUnit::H] }
    fn valid_str_units() -> &'static [&'static str] { &["s", "sec", "second", "min", "minute", "h", "hr", "hour"] }
}