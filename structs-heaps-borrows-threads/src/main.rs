use clap::Parser;
use std::thread;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[arg(short, long)]
    user_meaning: String
}

struct Meaning {
    number: i32,
    text: String,
}

fn main() {
    let cli = Cli::parse();
    let immutable = Meaning{ number: 42, text: cli.user_meaning};

    // this works because ownership is still in `immutable` after both borrows
    // which is dropped at the end of this scope
    print_meaning_number(&immutable);
    print_meaning_text(&immutable);

    let mut children = vec![];

    children.push(thread::spawn(move || {
        print_meaning_number(&immutable);
        // immutable dropped here
    }));

    // this doesn't work because ownership is moved twice. Which scope is responsible
    // for the drop?
    children.push(thread::spawn(move || {
        print_meaning_text(&immutable);
    }));

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}

fn print_meaning_text(m: &Meaning) {
    println!("The meaning of life is {}", m.text)
}

fn print_meaning_number(m: &Meaning) {
    println!("The meaning of life is {}", m.number)
}