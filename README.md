# RustQLi 🛡️

A modular, custom SQL Injection (SQLi) scanner written in **Rust** as a security proof-of-concept (PoC). Designed to detect parameter vulnerabilities via error-based signaling (HTTP 5xx server responses).

## Features

- **CLI Interface:** Built with `clap` for clean, robust command-line argument parsing.
- **URL Parsing:** Automatically extracts, maps, and iterates through query parameters using the `url` crate.
- **Targeted Fuzzing:** Safely injects single-quote payloads (`%27`) into each discovered parameter individually to isolate vulnerabilities.
- **Error-Based Signaling:** Flags potential SQL injection vulnerabilities by monitoring for HTTP 5xx server error status codes.

## Project Structure

```text
dsss-rs/
├── Cargo.toml      # Project dependencies (reqwest, clap, url)
├── src/
│   └── main.rs     # Core scanner logic and request handling
└── README.md       # Project documentation
