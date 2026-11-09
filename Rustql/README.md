# Rustql 🛡️

A modular, custom SQL Injection (SQLi) scanner written in **Rust** as a security proof-of-concept (PoC). Designed to detect parameter vulnerabilities via error-based signaling (HTTP 5xx server responses).

---

## Features

* Fast, memory-safe execution powered by Rust.
* Automated CLI parameter fuzzing.
* Error-based response inspection.

---

## Installation

1. Ensure you have **Rust and Cargo** installed (via [rustup](https://rustup.rs/)).
2. Clone the repository and navigate into the tool directory:
   ```bash
   git clone [https://github.com/kaymcneil1981-code/Scripts.git](https://github.com/kaymcneil1981-code/Scripts.git)
   cd Scripts/Rustql

## Build

1. Cargo build --release
2. Expected Output
    Compiling rustql v0.1.0 (/home/user/Scripts/Rustql)
        Finished `release` profile [optimized] target(s) in 2.14s
3. Location
   The compiled binary will be located at ./target/release/rustql.
   
## Execution

An example of the command to execute the script

1. Run the scanner against a target URL with a query parameter to test:
  ./target/release/rustql -u "[http://target.local/vuln.php?id=1](http://target.local/vuln.php?id=1)"

## Output example

[+] Initializing Rustql v0.1.0...
[+] Target: [http://target.local/vuln.php?id=1](http://target.local/vuln.php?id=1)
[+] Fuzzing parameter: id
[->] Testing payload: ' OR '1'='1
[!] Potential Vulnerability Detected!
[!] Status: 500 Internal Server Error
[!] Reason: Database syntax error signature matched in response body.
[+] Scan completed. 1 potential vulnerability found.

