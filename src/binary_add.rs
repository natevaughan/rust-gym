use std::env;

// add two binary characters and output as a char
fn main() {
    let usage_message: String = "Usage ./binary_add <first_binary_string> <second_binary_string>".to_owned();
    let mut bin1 = env::args().nth(1).expect(&usage_message);
    let bin2 = env::args().nth(2).expect(&usage_message);
    let bin3 = bin1.push_str(&bin2);
    println!("{} + {} = {:?}", &bin1, &bin2, bin3)
}