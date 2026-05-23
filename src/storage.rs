use std::fmt;
use std::str::FromStr;
use crate::errors::{AppError, suggest_unit};
use crate::unit::LinearUnit;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StorageUnit { B, Kb, Mb, Gb }

impl FromStr for StorageUnit {
    type Err = AppError;
    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        match unit.to_lowercase().as_str() {
            "b" | "byte" | "bytes" => Ok(StorageUnit::B),
"kb" | "kilobyte" | "kilobytes" => Ok(StorageUnit::Kb),
"mb" | "megabyte" | "megabytes" => Ok(StorageUnit::Mb),
"gb" | "gigabyte" | "gigabytes" => Ok(StorageUnit::Gb),
            _ => { let suggestion = suggest_unit(unit, Self::valid_str_units()); match suggestion { Some(s) => Err(AppError::InvalidUnitWithSuggestion(unit.to_string(), s)), None => Err(AppError::InvalidUnit(unit.to_string())) } }
        }
    }
}

impl fmt::Display for StorageUnit { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { match self { StorageUnit::B => write!(f, "B"), StorageUnit::Kb => write!(f, "KB"), StorageUnit::Mb => write!(f, "MB"), StorageUnit::Gb => write!(f, "GB") } } }

impl LinearUnit for StorageUnit {
    fn to_base(&self, value: f64) -> f64 { match self { StorageUnit::B => value, StorageUnit::Kb => value * 1024.0, StorageUnit::Mb => value * 1_048_576.0, StorageUnit::Gb => value * 1_073_741_824.0 } }
    fn convert_from_base(&self, value: f64) -> f64 { match self { StorageUnit::B => value, StorageUnit::Kb => value / 1024.0, StorageUnit::Mb => value / 1_048_576.0, StorageUnit::Gb => value / 1_073_741_824.0 } }
    fn variants() -> &'static [Self] { &[StorageUnit::B, StorageUnit::Kb, StorageUnit::Mb, StorageUnit::Gb] }
    fn valid_str_units() -> &'static [&'static str] { &["b", "byte", "bytes", "kb", "kilobyte", "kilobytes", "mb", "megabyte", "megabytes", "gb", "gigabyte", "gigabytes"] }
}