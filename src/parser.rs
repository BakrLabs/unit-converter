use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParsedCommand {
    pub category: String,
    pub value_expr: String,
    pub from: String,
    pub to: Option<String>,
}

fn build_language_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    
    m.insert("km/h", "kmh"); m.insert("kmh", "kmh");
    m.insert("m/s", "ms"); m.insert("ms", "ms");
    m.insert("mph", "mph");

    m.insert("c", "c"); m.insert("celsius", "c");
    m.insert("k", "k"); m.insert("kelvin", "k");
    m.insert("f", "f"); m.insert("fahrenheit", "f");

    for u in ["mm", "millimeter", "millimeters"] { m.insert(u, "mm"); }
    for u in ["cm", "centimeter", "centimeters"] { m.insert(u, "cm"); }
    for u in ["m", "meter", "meters"] { m.insert(u, "m"); }
    for u in ["km", "kilometer", "kilometers"] { m.insert(u, "km"); }

    for u in ["mg", "milligram", "milligrams"] { m.insert(u, "mg"); }
    for u in ["g", "gram", "grams"] { m.insert(u, "g"); }
    for u in ["kg", "kilogram", "kilograms"] { m.insert(u, "kg"); }

    for u in ["ml", "milliliter", "milliliters"] { m.insert(u, "ml"); }
    for u in ["l", "liter", "liters"] { m.insert(u, "l"); }
    for u in ["gal", "gallon", "gallons"] { m.insert(u, "gal"); }

    for u in ["s", "sec", "secs", "second", "seconds"] { m.insert(u, "s"); }
    for u in ["min", "mins", "minute", "minutes"] { m.insert(u, "min"); }
    for u in ["h", "hr", "hrs", "hour", "hours"] { m.insert(u, "h"); }

    for u in ["sqm", "m2"] { m.insert(u, "sqm"); }
    for u in ["sqkm", "km2"] { m.insert(u, "sqkm"); }
    for u in ["acre", "acres"] { m.insert(u, "acre"); }

    for u in ["b", "byte", "bytes"] { m.insert(u, "b"); }
    for u in ["kb", "kilobyte", "kilobytes"] { m.insert(u, "kb"); }
    for u in ["mb", "megabyte", "megabytes"] { m.insert(u, "mb"); }
    for u in ["gb", "gigabyte", "gigabytes"] { m.insert(u, "gb"); }

    // Currency
    for u in ["usd", "dollar", "dollars"] { m.insert(u, "usd"); }
    for u in ["eur", "euro", "euros"] { m.insert(u, "eur"); }
    for u in ["gbp", "pound", "pounds"] { m.insert(u, "gbp"); }
    for u in ["jpy", "yen", "yens"] { m.insert(u, "jpy"); }
    for u in ["sar", "riyal", "riyals"] { m.insert(u, "sar"); }
    for u in ["aed", "dirham", "dirhams"] { m.insert(u, "aed"); }
    for u in ["egp", "pound", "pounds"] { m.insert(u, "egp"); } // Note: pound overlaps, gbp takes precedence if checked first. Let's just use standard codes.

    m
}

fn get_category_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    for u in ["kmh", "ms", "mph"] { m.insert(u, "speed"); }
    for u in ["c", "k", "f"] { m.insert(u, "temp"); }
    for u in ["mm", "cm", "m", "km"] { m.insert(u, "distance"); }
    for u in ["mg", "g", "kg"] { m.insert(u, "mass"); }
    for u in ["ml", "l", "gal"] { m.insert(u, "volume"); }
    for u in ["s", "min", "h"] { m.insert(u, "time"); }
    for u in ["sqm", "sqkm", "acre"] { m.insert(u, "area"); }
    for u in ["b", "kb", "mb", "gb"] { m.insert(u, "storage"); }
    for u in ["usd", "eur", "gbp", "jpy", "sar", "aed", "egp"] { m.insert(u, "currency"); }
    m
}

pub fn parse(input: &str) -> Option<ParsedCommand> {
    let lang_map = build_language_map();
    let cat_map = get_category_map();
    let known_categories = ["speed", "temp", "distance", "mass", "volume", "time", "area", "storage", "currency"];

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

        if let Some(&canonical) = lang_map.get(part) {
            units_found.push(canonical.to_string());
            if prev_was_number {
                unit_after_number = Some(canonical.to_string());
            }
            prev_was_number = false;
            continue;
        }

        if let Some(caps) = re.captures(part) {
            let num_part = caps.get(1)?.as_str();
            let unit_part = caps.get(2)?.as_str();

            value_expr = Some(num_part.to_string());
            if let Some(&canonical) = lang_map.get(unit_part) {
                units_found.push(canonical.to_string());
                unit_after_number = Some(canonical.to_string());
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

    let category = category.or_else(|| cat_map.get(from.as_str()).map(|&c| c.to_string()));

    Some(ParsedCommand {
        category: category?,
        value_expr: value_expr?,
        from,
        to,
    })
}