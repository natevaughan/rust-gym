use std::fmt;

fn main() {
    let a = Item(1, "stick".to_string(), "Butter".to_string());
    println!("{}", a)
}

struct Item(u8, String, String);

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}