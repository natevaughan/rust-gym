use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please guess the password. Usage: ");
        eprintln!("cargo run <guess>");
        return
    }
    let guess = Guess { value: args[1].to_owned() };
    println!("Your guess was {guess:?}");
    let wisely = if guess.value == "foo" { "wisely!" } else { "poorly 💀" };
    println!("You have chosen... {0}", wisely);
}

#[derive(Debug)]
struct Guess {
    value: String
}
