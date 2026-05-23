use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParsedCommand {
    pub category: String,
    pub value_expr: String,
    pub from: String,
    pub to: Option<String>,
}

// خريطة تربط كل وحدة بتصنيفها (هذا هو عقل البرنامج الذكي)
fn get_unit_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();

    // Speed
    for u in ["kmh", "km/h", "ms", "m/s", "mph"] {
        m.insert(u, "speed");
    }
    // Temp
    for u in ["c", "k", "f", "celsius", "kelvin", "fahrenheit"] {
        m.insert(u, "temp");
    }
    // Distance
    for u in [
        "mm",
        "cm",
        "m",
        "km",
        "millimeter",
        "centimeter",
        "meter",
        "kilometer",
    ] {
        m.insert(u, "distance");
    }
    // Mass
    for u in ["mg", "g", "kg", "milligram", "gram", "kilogram"] {
        m.insert(u, "mass");
    }
    // Volume
    for u in ["ml", "l", "gal", "milliliter", "liter", "gallon"] {
        m.insert(u, "volume");
    }
    // Time
    for u in ["s", "min", "h", "sec", "second", "minute", "hr", "hour"] {
        m.insert(u, "time");
    }
    // Area
    for u in ["sqm", "sqkm", "acre", "m2", "km2"] {
        m.insert(u, "area");
    }
    // Storage
    for u in [
        "b", "kb", "mb", "gb", "byte", "kilobyte", "megabyte", "gigabyte",
    ] {
        m.insert(u, "storage");
    }

    m
}

pub fn parse(input: &str) -> Option<ParsedCommand> {
    let unit_map = get_unit_map();
    let known_categories = [
        "speed", "temp", "distance", "mass", "volume", "time", "area", "storage",
    ];

    let mut value_expr: Option<String> = None;
    let mut units_found: Vec<String> = Vec::new();
    let mut category: Option<String> = None;

    // تعبير نمطي (Regex) لفصل الأرقام والعمليات الحسابية عن الوحدات
    // مثال: "5km" تصبح ("5", "km") | "5+3m" تصبح ("5+3", "m")
    let re = Regex::new(r"^([\d\.\+\-\*\/\(\)e]+)([a-zA-Z°\/]+)$").ok()?;

    let input_lower = input.to_lowercase();
    let parts: Vec<&str> = input_lower.split_whitespace().collect();

    for part in parts {
        // 1. هل هذه الكلمة هي تصنيف؟ (مثل speed, distance)
        if known_categories.contains(&part) {
            category = Some(part.to_string());
            continue;
        }

        // 2. هل هذه الكلمة وحدة قياس معروفة؟
        if unit_map.contains_key(part) {
            units_found.push(part.to_string());
            continue;
        }

        // 3. هل الكلمة مدمجة (رقم + وحدة) مثل "5km"؟
        if let Some(caps) = re.captures(part) {
            let num_part = caps.get(1)?.as_str();
            let unit_part = caps.get(2)?.as_str();

            value_expr = Some(num_part.to_string());
            if unit_map.contains_key(unit_part) {
                units_found.push(unit_part.to_string());
            }
            continue;
        }

        // 4. هل الكلمة تحتوي على أرقام أو عمليات حسابية (وليست وحدة)؟
        if part.chars().any(|c| c.is_ascii_digit()) {
            value_expr = Some(part.to_string());
        }
    }

    // التحقق من أننا وجدنا رقماً ووحدة واحدة على الأقل
    if value_expr.is_none() || units_found.is_empty() {
        return None;
    }

    let from = units_found.first()?.to_string();
    let to = units_found.get(1).map(|s| s.to_string()); // الوحدة الثانية اختيارية

    // إذا لم يحدد المستخدم التصنيف صراحة، استنتجه من الوحدة
    let category = category.or_else(|| unit_map.get(from.as_str()).map(|&c| c.to_string()));

    Some(ParsedCommand {
        category: category?,
        value_expr: value_expr?,
        from,
        to,
    })
}
