use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please guess the password. Usage: ");
        eprintln!("cargo run <guess>");
        return
    }
    let guess = &args[1];
    println!("Your guess was {guess}");
    let wisely = if guess == "foo" { "wisely!" } else { "poorly 💀" };
    println!("You have chosen... {0}", wisely);
}
