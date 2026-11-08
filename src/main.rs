use clap::Parser;
use reqwest::blocking::Client;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about = "Damn Small SQLi Scanner in Rust")]
struct Args {
    #[arg(short, long)]
    url: String,
}

fn main() {
    let args = Args::parse();
    let client = Client::new();

    println!("[+] Initializing scan against target: {}", args.url);

    let parsed_url = match Url::parse(&args.url) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("[-] Invalid URL provided: {}", e);
            return;
        }
    };

    let pairs: Vec<(String, String)> = parsed_url.query_pairs().into_owned().collect();

    if pairs.is_empty() {
        println!("[-] Warning: No GET parameters found to test (e.g. ?id=1)");
        return;
    }

    println!("[+] Discovered {} query parameter(s). Starting payload injection tests...", pairs.len());

    // Simple test payload (classic single quote)
    let payload = "'";

    for (key, _original_value) in &pairs {
        println!("[*] Testing Parameter: '{}'", key);

        // Construct modified query parameters
        let mut test_url = parsed_url.clone();
        {
            let mut query_pairs = test_url.query_pairs_mut();
            query_pairs.clear();

            for (k, v) in &pairs {
                if k == key {
                    query_pairs.append_pair(k, &format!("{}{}", v, payload));
                } else {
                    query_pairs.append_pair(k, v);
                }
            }
        }

        println!("    -> Fuzzing URL: {}", test_url);
        match client.get(test_url.as_str()).send() {
            Ok(resp) => {
                let status = resp.status();
                println!("    -> Response Status: {}", status);

                if status.is_server_error() {
                    println!("[!] POTENTIAL SQLI VULNERABILITY DETECTED on parameter '{}' (Server error 5xx)", key);
                }
            }
            Err(e) => {
                eprintln!("    -> Request failed: {}", e);
            }
        }
    }
}
