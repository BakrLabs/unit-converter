# 🧠 Smart Unit Converter

A context-aware, highly intuitive command-line unit converter written in Rust. 
It doesn't just calculate; it understands what you mean. Type naturally, use math expressions, and let the smart engine do the rest!

[![crates.io](https://img.shields.io/crates/v/smart-unit-converter.svg)](https://crates.io/crates/smart-unit-converter)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Clippy](https://img.shields.io/badge/Clippy-Passed-green?style=for-the-badge)

---

## ✨ Features

- **🧠 Context-Aware Parsing:** No strict syntax required! It extracts the data it needs from your sentence. 
  - *Example:* `how many meters are in 5 kilometers`
- **🧮 Built-in Math Engine:** Calculate and convert at the same time using the `evalexpr` engine.
  - *Example:* `distance 5+3 km m` or `speed 100/2 mph kmh`
- **🔄 Auto-Conversion:** Omit the target unit to see conversions to **all** other units in the category.
  - *Example:* `5 km` converts to meters, cm, and mm simultaneously.
- **🤖 Smart Error Suggestions:** Made a typo? The fuzzy matcher will suggest the correct unit.
  - *Example:* `distance 5 kilo m` → `Invalid unit: 'kilo'. Did you mean 'kilometer'?`
- **📐 Smart Number Formatting:** Automatically adjusts decimal precision for tiny numbers (e.g., `0.00001 km`) and large numbers.
- **⏳ Persistent History:** Use Up/Down arrows to navigate command history (powered by `rustyline`). History survives app restarts!
- **⚡ Dual Mode:** Use it interactively (REPL) or directly from the command line (CLI) via `clap`.

---

## 🚀 Installation

You can install it directly via Cargo:

```bash
cargo install smart-unit-converter
```

Once installed, run it using the command:

```bash
suc
```

---

## 💻 Usage

### Interactive Mode (REPL)
Run the app and start typing naturally:

```bash
 $ suc
=== Smart Unit Converter ===
Type 'help' for examples | Type 'exit' to quit

> speed 72 kmh to ms
72 km/h → 20 m/s

> how many meters are in 5 kilometers
5 km → 5000 m

> convert 100 c to f
100 °C → 212 °F

> distance 5+3 km m
8 km → 8000 m

> 1 cm km
1 cm → 0.00001 km
```

### CLI Mode (One-liner)
You can also use it directly from the terminal without entering the REPL:

```bash
 $ suc speed 72 kmh ms
72 km/h → 20 m/s

 $ suc distance 10 km
10 km → 10000 m
10 km → 1000000 cm
10 km → 10000000 mm
```

---

## 🛠️ Architecture & Tech Stack

This project is built with software engineering best practices in mind:

- **Traits for DRY Code:** The `LinearUnit` trait abstracts the conversion logic. Adding a new category requires adding just one file.
- **Robust Error Handling:** Uses the `thiserror` crate to define custom, user-friendly `AppError` types.
- **Regex Entity Extraction:** The parser uses `regex` to decouple numbers from units, allowing flexible sentence structures.
- **Math Evaluation:** Integrates `evalexpr` safely, handling both Integer and Float returns seamlessly.

### Crates Used
- [`clap`](https://crates.io/crates/clap) - CLI argument parsing
- [`rustyline`](https://crates.io/crates/rustyline) - Readline implementation for history/navigation
- [`regex`](https://crates.io/crates/regex) - Context-aware string parsing
- [`evalexpr`](https://crates.io/crates/evalexpr) - Math expression evaluation
- [`thiserror`](https://crates.io/crates/thiserror) - Error handling derivation
- [`colored`](https://crates.io/crates/colored) - Terminal color output

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! 
Feel free to check the [issues page](https://github.com/BakrLabs/unit-converter/issues).

## 📝 License

This project is [Apache-2.0](./LICENSE) licensed.