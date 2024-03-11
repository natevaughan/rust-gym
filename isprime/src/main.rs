use std::env;

fn main() {
    let arg = env::args().nth(1).expect("Usage: cargo run <positive_integer_to_check>");
    let n: u32 = arg.parse().expect("Please specify a positive integer");
    is_prime(n);
}

fn is_prime(n: u32) -> bool {
    let mut is_prime = true;
    for divisor in 2..n {
        if n % divisor == 0 {
            println!("{} is not prime because it is divisible by {}", n, divisor);
            is_prime = false
        }
        let divided = n / divisor;
        if divisor > divided {
            println!("{} is prime. Checked to {}", n, divisor);
            break
        }
    }
    is_prime
} 

#[cfg(test)]
mod tests {
    use crate::is_prime;

    #[test]
    fn test_1() {
        let res = is_prime(7);
        assert_eq!(true, res);
    }

    #[test]
    fn test_2() {
        let res = is_prime(143);
        assert_eq!(false, res);
    }
}
