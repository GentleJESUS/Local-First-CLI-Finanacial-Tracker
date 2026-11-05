[package]
name = "fintrack"
version = "1.0.0"
edition = "2026"

[dependencies]
chrono = "0.4.42"
clap = { version = "4.5.53", features = ["derive"] }
dirs = "6.0.0"
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.148"
strum = { version = "0.26", features = ["derive"] }
