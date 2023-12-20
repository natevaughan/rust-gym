use std::env;

fn main() {
    let arg: usize = env::args()
        .nth(1)
        .expect("Usage: cargo run <integer>")
        .parse()
        .expect("Usage: cargo run <integer>");
    println!("CS50 Mario! See HarvardX / CS50. Height {}", arg);

    let symbol: String = env::args()
        .nth(2)
        .unwrap_or("#".to_string());

    for i in 0..arg {
        let o = arg - i;
        print!("{symbol:>o$}");
        for _ in 0..i {
            print!("{}", symbol);
        }
        println!("");
    }
}
