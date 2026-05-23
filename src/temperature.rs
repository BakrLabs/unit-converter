use crate::errors::{suggest_unit, AppError};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemperatureUnit {
    C,
    K,
    F,
}

impl FromStr for TemperatureUnit {
    type Err = AppError;
    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        match unit.to_lowercase().as_str() {
            "c" | "celsius" => Ok(TemperatureUnit::C),
            "k" | "kelvin" => Ok(TemperatureUnit::K),
            "f" | "fahrenheit" => Ok(TemperatureUnit::F),
            _ => {
                let suggestion = suggest_unit(unit, Self::valid_str_units());
                match suggestion {
                    Some(s) => Err(AppError::InvalidUnitWithSuggestion(unit.to_string(), s)),
                    None => Err(AppError::InvalidUnit(unit.to_string())),
                }
            }
        }
    }
}

impl fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemperatureUnit::C => write!(f, "°C"),
            TemperatureUnit::K => write!(f, "K"),
            TemperatureUnit::F => write!(f, "°F"),
        }
    }
}

impl TemperatureUnit {
    pub fn variants() -> &'static [Self] {
        &[TemperatureUnit::C, TemperatureUnit::K, TemperatureUnit::F]
    }
    pub fn valid_str_units() -> &'static [&'static str] {
        &["c", "celsius", "k", "kelvin", "f", "fahrenheit"]
    }
}

pub fn convert(from: TemperatureUnit, to: TemperatureUnit, value: f64) -> Result<f64, AppError> {
    if from == to {
        return Ok(value);
    }
    match (from, to) {
        (TemperatureUnit::C, TemperatureUnit::K) => Ok(value + 273.15),
        (TemperatureUnit::K, TemperatureUnit::C) => Ok(value - 273.15),
        (TemperatureUnit::C, TemperatureUnit::F) => Ok((value * 9.0 / 5.0) + 32.0),
        (TemperatureUnit::F, TemperatureUnit::C) => Ok((value - 32.0) * 5.0 / 9.0),
        (TemperatureUnit::F, TemperatureUnit::K) => Ok((value - 32.0) * 5.0 / 9.0 + 273.15),
        (TemperatureUnit::K, TemperatureUnit::F) => Ok((value - 273.15) * 9.0 / 5.0 + 32.0),
        _ => Err(AppError::UnsupportedConversion),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        let result = convert(TemperatureUnit::C, TemperatureUnit::F, 0.0).unwrap();
        assert!((result - 32.0).abs() < 1e-10);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        let result = convert(TemperatureUnit::C, TemperatureUnit::K, 100.0).unwrap();
        assert!((result - 373.15).abs() < 1e-10);
    }
}
