RANDOM Generator
===================
[![Build Status](https://travis-ci.com/marirs/random-rs.svg?branch=main)](https://travis-ci.com/marirs/random-rs)

A Random generator for specific use-cases.

## Features
- Datetime generator for DateTime<Utc> between start & end dates
- TimeZone generator & lookup (Generates a random timezone and/or lookup for timezoneas)

## Requirements
- Rust

## Usage
- with `all features`
```toml
[dependencies]
random = { git = "https://github.com/marirs/random-rs", branch = "main" }

[dependencies.random]
features = ["random-all"]
```

- with `specific feature` datetime generator
```toml
[dependencies.random]
features = ["timegenerator"]
```

## Compile
- compile with `all features`
```bash
cargo b --features "random-all"
```
- compile with `specific feature`: `cargo b --features "<feature_name>"`; eg:
```bash
cargo b --features "timegenerate tz"
```
compiles with both timegenerate and tz features

## Running examples from the example folder
- Example of the Timezone generator
```bash
cargo run --example tz --features "tz"
   Compiling random v0.1.0 (/Users/sgp/Documents/DEV/Repos/random-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 1.40s
     Running `target/debug/examples/tz`
Ok(Some("Asia/Calcutta,Asia/Kolkata"))
Tz {
    alpha_2_code: "TL",
    alpha_3_code: "TLS",
    continent: "Asia",
    capital: "Dili",
    name: "East Timor",
    timezones: [
        "Asia/Dili",
    ],
}
```