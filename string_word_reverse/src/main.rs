use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    println!("original arguments: {}", args.join(" "));
    let words = &mut args[1..];
    println!("word arguments: {}", words.join(" "));
    words.reverse();
    println!("reversed arguments: {}", words.join(" "));
    println!("and finally let's check the original arguments: {}", args.join(" "));
}
