use std::env;
use std::fmt;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please guess the password. Usage: ");
        eprintln!("cargo run <guess>");
        return
    }
    let guess = Guess { value: args[1].to_string() };
    println!("Your guess was {guess}");
    let (msg, _) = check(guess);
    println!("You have chosen... {}", msg);
}

#[derive(Debug)]
struct Guess {
    pub value: String
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn check(guess: Guess) -> (String, bool) {
    let result = guess.value == "foo";
    let wisely = if result { "wisely!" } else { "poorly 💀" };
    (wisely.to_string(), result)
}