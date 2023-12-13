use std::env;

fn main() {
    let o = env::args().nth(1);
    let mut error_msg = "Invalid non-numeric argument".to_owned();
    if o == None {
        println!("Using default value of 100");
    } else {
        error_msg.push_str(o.as_ref().unwrap());
    }
    let number: i32 = o.unwrap_or("100".to_owned()).parse::<i32>().expect(&error_msg);

    for n in 1..=number {
        let mut message = "".to_owned();
        if n % 3 == 0 {
            message.push_str("Fizz");
        }
        if n % 5 == 0 {
            message.push_str("Buzz");
        }
        if n % 7 == 0 {
            message.push_str("Bing!");
        }
        if message == "" {
            message.push_str(&n.to_string());
        }
        println!("{}", message)
    }
}