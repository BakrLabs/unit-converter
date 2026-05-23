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
}

// دالة ذكية للبحث عن أقرب وحدة مطابقة (Fuzzy Matching بسيط)
pub fn suggest_unit(input: &str, valid_units: &[&str]) -> Option<String> {
    let input_lower = input.to_lowercase();

    // 1. بحث عن بداية مطابقة (مثل kil -> kilometer)
    for &unit in valid_units {
        if unit.starts_with(&input_lower) {
            return Some(unit.to_string());
        }
    }

    // 2. بحث عن احتواء (مثل meter -> kilometer)
    for &unit in valid_units {
        if unit.contains(&input_lower) {
            return Some(unit.to_string());
        }
    }

    None
}
