use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: cargo run <noun> <verb>");
        return
    }
    let noun = &args[1];
    let verb = &args[2];

    println!("There once was a {noun} from Nantucket who liked to {verb}");
}
  