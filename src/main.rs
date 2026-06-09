mod area;
mod distance;
mod errors;
mod history;
mod mass;
mod parser;
mod speed;
mod storage;
mod temperature;
mod time;
mod unit;
mod volume;

use clap::Parser;
use colored::*;
use errors::AppError;
use evalexpr::{eval, Value};
use history::History;
use parser::parse;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::str::FromStr;
use unit::{convert_linear, convert_to_all_linear, LinearUnit};

#[derive(Parser, Debug)]
#[command(name = "unit_converter", about = "Advanced Unit Converter", version)]
struct Cli {
    category: Option<String>,
    value_expr: Option<String>,
    from: Option<String>,
    to: Option<String>,
}
/// Evaluates a mathematical expression string and returns the result as f64.
/// Supports both integers and floats safely.
fn evaluate_math(expr: &str) -> Result<f64, AppError> {
    match eval(expr) {
        Ok(Value::Float(f)) => Ok(f),
        Ok(Value::Int(i)) => Ok(i as f64),
        Ok(_) => Err(AppError::MathError("Result is not a number".to_string())),
        Err(_) => Err(AppError::MathError(format!(
            "'{}' is not a valid number or math expression",
            expr
        ))),
    }
}
/// Smart number formatter that adjusts decimal precision.
/// Displays up to 8 decimals for tiny numbers and removes trailing zeros for large ones.
fn format_number(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }

    let abs_val = value.abs();
    let formatted = if abs_val < 0.01 {
        format!("{:.8}", value)
    } else if abs_val >= 1000.0 {
        format!("{:.2}", value)
    } else {
        format!("{:.4}", value)
    };

    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}
/// Supports both interactive REPL mode and direct CLI arguments.
fn main() {
    let cli = Cli::parse();
    if let (Some(category), Some(value_expr), Some(from)) =
        (cli.category, cli.value_expr, cli.from)
    {
        let to = cli.to;
        match evaluate_math(&value_expr) {
            Ok(value) => execute_conversion(&category, value, &from, to.as_deref()),
            Err(e) => println!("{}", e.to_string().red()),
        }
        return;
    }

    let mut rl = DefaultEditor::new().expect("Failed to initialize editor");
    let mut history = History::new();
    let _ = rl.load_history("history.txt");

    println!("{}", "=== Smart Unit Converter ===".bright_green());
    println!("Type 'help' for examples | Type 'exit' to quit");

    loop {
        let prompt = format!("{}", "> ".bright_blue());
        let readline = rl.readline(&prompt);

        match readline {
            Ok(input) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(input);

                if input == "exit" {
                    println!("{}", "Goodbye!".red());
                    let _ = rl.save_history("history.txt");
                    break;
                }
                if input == "history" {
                    history.show();
                    continue;
                }
                if input == "list" {
                    println!("speed, temp, distance, mass, volume, time, area, storage");
                    continue;
                }

                                if input == "help" {
                    println!("{}", "╔══════════════════════════════════════════╗".bright_green());
                    println!("{}", "║        SMART UNIT CONVERTER HELP         ║".bright_green());
                    println!("{}", "╚══════════════════════════════════════════╝".bright_green());

                    println!("\n{}", "Supported Categories & Units:".bright_yellow());
                    println!("  {} speed     : kmh, km/h, ms, m/s, mph", "•".bright_cyan());
                    println!("  {} temp      : c, celsius, k, kelvin, f, fahrenheit", "•".bright_cyan());
                    println!("  {} distance  : mm, cm, m, km, millimeter(s), centimeter(s), meter(s), kilometer(s)", "•".bright_cyan());
                    println!("  {} mass      : mg, g, kg, milligram(s), gram(s), kilogram(s)", "•".bright_cyan());
                    println!("  {} volume    : ml, l, gal, milliliter(s), liter(s), gallon(s)", "•".bright_cyan());
                    println!("  {} time      : s, sec(s), min(s), h, hr(s), hour(s)", "•".bright_cyan());
                    println!("  {} area      : sqm, m2, sqkm, km2, acre(s)", "•".bright_cyan());
                    println!("  {} storage   : b, kb, mb, gb, byte(s), kilobyte(s), megabyte(s), gigabyte(s)", "•".bright_cyan());

                    println!("\n{}", "A context-aware tool! Just type naturally:".bright_yellow());
                    println!("  speed 72 kmh to ms");
                    println!("  how many meters are in 5 kilometers");
                    println!("  convert 100 celsius to fahrenheit");
                    println!("  5km m   (Short format)");
                    println!("  5km     (Auto-convert to all distance units)");

                    println!("\n{}", "Math Expressions:".bright_yellow());
                    println!("  distance 5+3 km m");
                    println!("  speed 100/2 mph kmh");

                    println!("\n{}", "Commands:".bright_yellow());
                    println!("  list, history, exit");
                    continue;
                }

                let command = match parse(input) {
                    Some(cmd) => cmd,
                    None => {
                        println!(
                            "{}",
                            "Hmm, I didn't understand that. Type 'help' for examples."
                                .yellow()
                        );
                        continue;
                    }
                };

                match evaluate_math(&command.value_expr) {
                    Ok(value) => {
                        history.add(input.to_string());
                        execute_conversion(
                            &command.category,
                            value,
                            &command.from,
                            command.to.as_deref(),
                        );
                    }
                    Err(e) => println!("{}", e.to_string().red()),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "^C".red());
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "Goodbye!".red());
                break;
            }
            Err(err) => {
                println!("{}", format!("Error: {:?}", err).red());
                break;
            }
        }
    }
    let _ = rl.save_history("history.txt");
}
/// Executes the conversion logic based on the category and units provided.
fn execute_conversion(category: &str, value: f64, from_str: &str, to_str: Option<&str>) {
    match category.to_lowercase().as_str() {
        "speed" => process_linear::<speed::SpeedUnit>(value, from_str, to_str),
        "distance" => process_linear::<distance::DistanceUnit>(value, from_str, to_str),
        "mass" => process_linear::<mass::MassUnit>(value, from_str, to_str),
        "volume" => process_linear::<volume::VolumeUnit>(value, from_str, to_str),
        "time" => process_linear::<time::TimeUnit>(value, from_str, to_str),
        "area" => process_linear::<area::AreaUnit>(value, from_str, to_str),
        "storage" => process_linear::<storage::StorageUnit>(value, from_str, to_str),
        "temp" => process_temperature(value, from_str, to_str),
        _ => println!("{}", "Unknown category".red()),
    }
}

fn process_linear<U: LinearUnit>(value: f64, from_str: &str, to_str: Option<&str>) {
    let from = match U::from_str(from_str) {
        Ok(u) => u,
        Err(e) => {
            println!("{}", e.to_string().red());
            return;
        }
    };

    match to_str {
        Some(t) => {
            let to = match U::from_str(t) {
                Ok(u) => u,
                Err(e) => {
                    println!("{}", e.to_string().red());
                    return;
                }
            };
            let result = convert_linear(from, to, value);
            println!(
                "{}",
                format!(
                    "{} {} → {} {}",
                    format_number(value),
                    from,
                    format_number(result),
                    to
                )
                .bright_green()
            );
        }
        None => {
            let results = convert_to_all_linear(from, value);
            for (unit, result) in results {
                println!(
                    "{}",
                    format!(
                        "{} {} → {} {}",
                        format_number(value),
                        from,
                        format_number(result),
                        unit
                    )
                    .bright_green()
                );
            }
        }
    }
}

fn process_temperature(value: f64, from_str: &str, to_str: Option<&str>) {
    let from = match temperature::TemperatureUnit::from_str(from_str) {
        Ok(u) => u,
        Err(e) => {
            println!("{}", e.to_string().red());
            return;
        }
    };

    match to_str {
        Some(t) => {
            let to = match temperature::TemperatureUnit::from_str(t) {
                Ok(u) => u,
                Err(e) => {
                    println!("{}", e.to_string().red());
                    return;
                }
            };
            match temperature::convert(from, to, value) {
                Ok(result) => println!(
                    "{}",
                    format!(
                        "{} {} → {} {}",
                        format_number(value),
                        from,
                        format_number(result),
                        to
                    )
                    .bright_green()
                ),
                Err(e) => println!("{}", e.to_string().red()),
            }
        }
        None => {
            for &to in temperature::TemperatureUnit::variants() {
                if to != from {
                    if let Ok(result) = temperature::convert(from, to, value) {
                        println!(
                            "{}",
                            format!(
                                "{} {} → {} {}",
                                format_number(value),
                                from,
                                format_number(result),
                                to
                            )
                            .bright_green()
                        );
                    }
                }
            }
        }
    }
}