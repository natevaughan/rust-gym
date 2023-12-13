use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage ./openai_fetch <openai_key> <path/to/prompt>");
        return ExitCode::from(1);
    }
    return ExitCode::from(0);
}