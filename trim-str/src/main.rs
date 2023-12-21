use std::env;
use substring::Substring;

fn main() {
    let s = env::args().nth(1).expect("usage: cargo run <string_to_trim>");
    println!("Trimmed string: {}", trim_str(&s))
}


fn trim_str(original: &str) -> &str {
    original.substring(1, original.len() - 1)
}