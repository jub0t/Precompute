extern crate colored;

use regex::Regex;
use std::time;
use std::{collections::HashMap, io::stdin};

use colored::*;
mod sha256;

pub fn is_valid_sha256_hash(input: &str) -> bool {
    let sha256_regex = Regex::new(r"^[0-9a-fA-F]{64}$").unwrap();
    sha256_regex.is_match(input)
}

fn generate_combinations(allowed: &str) -> Vec<String> {
    let letters: &str = allowed;
    let mut combinations = Vec::new();

    for c1 in letters.chars() {
        for c2 in letters.chars() {
            for c3 in letters.chars() {
                for c4 in letters.chars() {
                    let password = format!("{}{}{}{}", c1, c2, c3, c4);
                    combinations.push(password);
                }
            }
        }
    }

    combinations
}

fn main() {
    let mut store: HashMap<String, String> = HashMap::new();
    let mut sha = sha256::Sha256::new();

    let allowed = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    println!("{} {}", "ALLOWED CHARACTERS".blue().bold(), allowed);

    let start = time::Instant::now();
    let passwords = generate_combinations(allowed);
    println!(
        "{}: {} Words",
        format!(
            "[ELAPSED: {}ms] {}",
            start.elapsed().as_millis(),
            "COMBINATIONS"
        )
        .blue()
        .bold(),
        passwords.len()
    );

    let start = time::Instant::now();
    for pass in passwords {
        let init_bytes = pass.as_bytes();
        sha.update(init_bytes);
        let hash_data = sha.finalize();
        store.insert(sha256::digest_string(&hash_data), pass.to_owned());
        sha.reset();
    }

    println!(
        "{}: {} Hashes",
        format!(
            "[ELAPSED: {}ms] {}",
            start.elapsed().as_millis(),
            "CALCULATED"
        )
        .green()
        .bold(),
        store.len()
    );

    loop {
        println!("{} Sha256 Hash:", "ENTER".yellow().bold());

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Please enter a hash.");

        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }

        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }

        if is_valid_sha256_hash(&input) {
            let start = time::Instant::now();
            match store.get(input.to_lowercase().as_str()) {
                None => {
                    println!("No Record Found")
                }
                Some(data) => {
                    println!(
                        "{}: {}",
                        format!("[ELAPSED: {}ns] {}", start.elapsed().as_nanos(), "CRACKED")
                            .green()
                            .bold(),
                        data
                    );
                }
            }
        } else {
            println!("Invalid Hash");
        };
    }
}
