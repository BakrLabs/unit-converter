mod area;
mod currency;
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
#[command(name = "smart-unit-converter", about = "Advanced Unit Converter", version)]
struct Cli {
    category: Option<String>,
    value_expr: Option<String>,
    from: Option<String>,
    to: Option<String>,
}

fn evaluate_math(expr: &str) -> Result<f64, AppError> {
    match eval(expr) {
        Ok(Value::Float(f)) => Ok(f),
        Ok(Value::Int(i)) => Ok(i as f64),
        Ok(_) => Err(AppError::MathError("Result is not a number".to_string())),
        Err(_) => Err(AppError::MathError(format!("'{}' is not a valid number or math expression", expr))),
    }
}

fn format_number(value: f64) -> String {
    if value == 0.0 { return "0".to_string(); }
    let abs_val = value.abs();
    let formatted = if abs_val < 0.01 { format!("{:.8}", value) } else if abs_val >= 1000.0 { format!("{:.2}", value) } else { format!("{:.4}", value) };
    formatted.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let (Some(category), Some(value_expr), Some(from)) = (cli.category, cli.value_expr, cli.from) {
        let to = cli.to;
        match evaluate_math(&value_expr) {
            Ok(value) => execute_conversion(&category, value, &from, to.as_deref()).await,
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
                if input.is_empty() { continue; }
                let _ = rl.add_history_entry(input);

                if input == "exit" { 
                    println!("{}", "See you later!".bright_green()); 
                    let _ = rl.save_history("history.txt");
                    break; 
                }
                if input == "history" { history.show(); continue; }
                if input == "list" {
                    println!("{}", "I can convert these categories:".bright_yellow());
                    println!("  speed, temp, distance, mass, volume, time, area, storage, currency");
                    continue;
                }

                if input == "help" {
                    println!("{}", "╔══════════════════════════════════════════╗".bright_green());
                    println!("{}", "║       SMART UNIT CONVERTER HELP          ║".bright_green());
                    println!("{}", "╚══════════════════════════════════════════╝".bright_green());

                    println!("\n{}", "Talk to me naturally:".bright_yellow());
                    println!("  speed 72 kmh to ms");
                    println!("  how many meters are in 5 kilometers");
                    println!("  100 usd to eur");
                    println!("  5km m   (Short format)");
                    println!("  5km     (Auto-convert to all distance units)");

                    println!("\n{}", "You can do math too:".bright_yellow());
                    println!("  distance 5+3 km m");
                    println!("  speed 100/2 mph kmh");

                    println!("\n{}", "Live Currency Codes:".bright_yellow());
                    println!("  usd, eur, gbp, jpy, sar, aed, egp...");
                    continue;
                }

                let command = match parse(input) {
                    Some(cmd) => cmd,
                    None => { 
                        println!("{}", "I didn't quite catch that. Type 'help' to see what I understand.".yellow()); 
                        continue; 
                    }
                };

                match evaluate_math(&command.value_expr) {
                    Ok(value) => {
                        history.add(input.to_string());
                        execute_conversion(&command.category, value, &command.from, command.to.as_deref()).await;
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
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    let _ = rl.save_history("history.txt");
}

async fn execute_conversion(category: &str, value: f64, from_str: &str, to_str: Option<&str>) {
    match category.to_lowercase().as_str() {
        "speed" => process_linear::<speed::SpeedUnit>(value, from_str, to_str),
        "distance" => process_linear::<distance::DistanceUnit>(value, from_str, to_str),
        "mass" => process_linear::<mass::MassUnit>(value, from_str, to_str),
        "volume" => process_linear::<volume::VolumeUnit>(value, from_str, to_str),
        "time" => process_linear::<time::TimeUnit>(value, from_str, to_str),
        "area" => process_linear::<area::AreaUnit>(value, from_str, to_str),
        "storage" => process_linear::<storage::StorageUnit>(value, from_str, to_str),
        "temp" => process_temperature(value, from_str, to_str),
        "currency" => process_currency(value, from_str, to_str).await,
        _ => println!("{}", "Unknown category".red()),
    }
}

fn process_linear<U: LinearUnit>(value: f64, from_str: &str, to_str: Option<&str>) {
    let from = match U::from_str(from_str) {
        Ok(u) => u,
        Err(e) => { println!("{}", e.to_string().red()); return; }
    };

    match to_str {
        Some(t) => {
            let to = match U::from_str(t) {
                Ok(u) => u,
                Err(e) => { println!("{}", e.to_string().red()); return; }
            };
            let result = convert_linear(from, to, value);
            println!("{}", format!("{} {} → {} {}", format_number(value), from, format_number(result), to).bright_green());
        }
        None => {
            let results = convert_to_all_linear(from, value);
            for (unit, result) in results {
                println!("{}", format!("{} {} → {} {}", format_number(value), from, format_number(result), unit).bright_green());
            }
        }
    }
}

fn process_temperature(value: f64, from_str: &str, to_str: Option<&str>) {
    let from = match temperature::TemperatureUnit::from_str(from_str) {
        Ok(u) => u,
        Err(e) => { println!("{}", e.to_string().red()); return; }
    };

    match to_str {
        Some(t) => {
            let to = match temperature::TemperatureUnit::from_str(t) {
                Ok(u) => u,
                Err(e) => { println!("{}", e.to_string().red()); return; }
            };
            match temperature::convert(from, to, value) {
                Ok(result) => println!("{}", format!("{} {} → {} {}", format_number(value), from, format_number(result), to).bright_green()),
                Err(e) => println!("{}", e.to_string().red()),
            }
        }
        None => {
            for &to in temperature::TemperatureUnit::variants() {
                if to != from {
                    if let Ok(result) = temperature::convert(from, to, value) {
                        println!("{}", format!("{} {} → {} {}", format_number(value), from, format_number(result), to).bright_green());
                    }
                }
            }
        }
    }
}

async fn process_currency(value: f64, from_str: &str, to_str: Option<&str>) {
    match to_str {
        Some(t) => {
            match currency::convert(from_str, t, value).await {
                Ok(result) => println!("{}", format!("{} {} → {} {}", format_number(value), from_str.to_uppercase(), format_number(result), t.to_uppercase()).bright_green()),
                Err(e) => println!("{}", e.to_string().red()),
            }
        }
        None => println!("{}", "Currency conversion requires a target unit (e.g., 100 usd to eur)".yellow()),
    }
}