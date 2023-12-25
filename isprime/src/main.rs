use std::env;

fn main() {
    let arg = env::args().nth(1).expect("Usage: cargo run <positive_integer_to_check>");
    let n: u64 = arg.parse().expect("Please specify a positive integer");
    for divisor in 2..n {
        if n % divisor == 0 {
            println!("{} is not prime because it is divisible by {}", n, divisor);
            return
        }
        let divided = n / divisor;
        if divisor > divided {
            println!("{} is prime. Checked to {}", n, divisor);
            break
        }
    }
}
