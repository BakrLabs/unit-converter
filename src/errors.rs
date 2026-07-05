use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid unit: '{0}'. Did you mean '{1}'?")]
    InvalidUnitWithSuggestion(String, String),
    
    #[error("Invalid unit: '{0}'")]
    InvalidUnit(String),

    #[error("Math evaluation error: {0}")]
    MathError(String),

    #[error("Unsupported conversion")]
    UnsupportedConversion,

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("API parsing error: {0}")]
    ParsingError(String),
}

pub fn suggest_unit(input: &str, valid_units: &[&str]) -> Option<String> {
    let input_lower = input.to_lowercase();
    for &unit in valid_units {
        if unit.starts_with(&input_lower) { return Some(unit.to_string()); }
    }
    for &unit in valid_units {
        if unit.contains(&input_lower) { return Some(unit.to_string()); }
    }
    None
}