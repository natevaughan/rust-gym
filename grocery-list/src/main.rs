use std::fmt;

fn main() {
    let a = Item(1, "stick", "Butter");
    println!("{}", a)
}

struct Item(u8, &'static str, &'static str);

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}