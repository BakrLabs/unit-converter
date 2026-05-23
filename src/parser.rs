use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParsedCommand {
    pub category: String,
    pub value_expr: String,
    pub from: String,
    pub to: Option<String>,
}

fn get_unit_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    // Speed
    for u in ["kmh", "km/h", "ms", "m/s", "mph"] { m.insert(u, "speed"); }
    // Temp
    for u in ["c", "k", "f", "celsius", "kelvin", "fahrenheit"] { m.insert(u, "temp"); }
    // Distance
    for u in ["mm", "cm", "m", "km", "millimeter", "centimeter", "meter", "kilometer", "millimeters", "centimeters", "meters", "kilometers"] { m.insert(u, "distance"); }
    // Mass
    for u in ["mg", "g", "kg", "milligram", "gram", "kilogram", "milligrams", "grams", "kilograms"] { m.insert(u, "mass"); }
    // Volume
    for u in ["ml", "l", "gal", "milliliter", "liter", "gallon", "milliliters", "liters", "gallons"] { m.insert(u, "volume"); }
    // Time
    for u in ["s", "min", "h", "sec", "second", "minute", "hr", "hour", "secs", "seconds", "mins", "minutes", "hrs", "hours"] { m.insert(u, "time"); }
    // Area
    for u in ["sqm", "sqkm", "acre", "m2", "km2", "acres"] { m.insert(u, "area"); }
    // Storage
    for u in ["b", "kb", "mb", "gb", "byte", "kilobyte", "megabyte", "gigabyte", "bytes", "kilobytes", "megabytes", "gigabytes"] { m.insert(u, "storage"); }
    
    m
}

pub fn parse(input: &str) -> Option<ParsedCommand> {
    let unit_map = get_unit_map();
    let known_categories = ["speed", "temp", "distance", "mass", "volume", "time", "area", "storage"];

    let mut value_expr: Option<String> = None;
    let mut units_found: Vec<String> = Vec::new();
    let mut unit_after_number: Option<String> = None;
    let mut category: Option<String> = None;
    let mut prev_was_number = false;

    let re = Regex::new(r"^([\d\.\+\-\*\/\(\)e]+)([a-zA-Z°\/]+)$").ok()?;

    let input_lower = input.to_lowercase();
    let parts: Vec<&str> = input_lower.split_whitespace().collect();

    for part in parts {
        if known_categories.contains(&part) {
            category = Some(part.to_string());
            prev_was_number = false;
            continue;
        }

        if unit_map.contains_key(part) {
            units_found.push(part.to_string());
            if prev_was_number {
                unit_after_number = Some(part.to_string());
            }
            prev_was_number = false;
            continue;
        }

        if let Some(caps) = re.captures(part) {
            let num_part = caps.get(1)?.as_str();
            let unit_part = caps.get(2)?.as_str();

            value_expr = Some(num_part.to_string());
            if unit_map.contains_key(unit_part) {
                units_found.push(unit_part.to_string());
                unit_after_number = Some(unit_part.to_string());
            }
            prev_was_number = false;
            continue;
        }

        if part.chars().any(|c| c.is_ascii_digit()) {
            value_expr = Some(part.to_string());
            prev_was_number = true;
            continue;
        }

        prev_was_number = false;
    }

    if value_expr.is_none() || units_found.is_empty() {
        return None;
    }
    let from = unit_after_number.or_else(|| units_found.first().cloned())?;
        let to = if units_found.len() > 1 {
        units_found.iter().find(|u| *u != &from).cloned()
    } else {
        None
    };

    let category = category.or_else(|| unit_map.get(from.as_str()).map(|&c| c.to_string()));

    Some(ParsedCommand {
        category: category?,
        value_expr: value_expr?,
        from,
        to,
    })
}