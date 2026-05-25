use std::fmt;
use std::str::FromStr;
use crate::errors::AppError;

/// A trait for units that follow a linear conversion scale (e.g., Distance, Mass).
/// Provides a standardized way to convert to and from a base unit.
pub trait LinearUnit: FromStr<Err = AppError> + fmt::Display + Copy + Clone + PartialEq + 'static {
    fn to_base(&self, value: f64) -> f64;
    fn convert_from_base(&self, value: f64) -> f64; // تم تغيير الاسم لإرضاء Clippy
    fn variants() -> &'static [Self];
    fn valid_str_units() -> &'static [&'static str];
}

pub fn convert_linear<U: LinearUnit>(from: U, to: U, value: f64) -> f64 {
    to.convert_from_base(from.to_base(value))
}

pub fn convert_to_all_linear<U: LinearUnit>(from: U, value: f64) -> Vec<(U, f64)> {
    U::variants()
        .iter()
        .filter(|&&u| u != from)
        .map(|&u| (u, convert_linear(from, u, value)))
        .collect()
}