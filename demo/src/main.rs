mod osha;

fn combinations(allowed: &str) -> Vec<String> {
    let letters = allowed;
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
    let now = std::time::Instant::now();
    let mut sha = osha::Sha256::new();

    let combos = combinations("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    println!("Length: {}", combos.len());

    for word in combos {
        sha.update(word.as_bytes());
        osha::digest_string(sha.finalize());
        sha.reset();
    }

    println!("{}", now.elapsed().as_millis());

    loop {}
}
