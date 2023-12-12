use std::env;
use std::collections::HashMap;

fn main() {
	let digit: String = env::args().nth(1).expect("No nth digit of the fibonacci sequence specified");

    let suffixes: HashMap<&str, &str> = HashMap::from([
        ("1", "st"),
        ("2", "nd"),
        ("3", "rd")
    ]);

    let number = digit.parse::<i32>().expect("Please specify a number representing the nth digit of the fibonacci sequence");

	println!("The {}{} digit of the fibonnacci sequence is {}", number, suffixes.get(&*digit).unwrap_or(&"th"), fibonacci(1, 0, 1, number))
}

fn fibonacci(layer: i32, before_prev: i32, prev: i32, depth: i32) -> i32 {
    if layer == depth {
        return prev
    }
    return fibonacci(layer + 1, prev, before_prev + prev, depth)
}