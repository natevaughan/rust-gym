use std::env;

fn main() {
    let n = env::args().nth(1).expect("Usage: cargo run <number_to_flip>");
    let i = n.parse::<i64>();
    match i {
        Ok(i2) =>  {
            println!("The opposite of {} is {}", i2, opposte(i2))
        },
        Err(_) => {
            let f = n.parse::<f64>().unwrap();
            println!("The opposite of {} is {}", f, opposte(f));
        },
    };
}

fn opposte<T: std::ops::Neg<Output = T>>(n: T) -> T {
    -n
}