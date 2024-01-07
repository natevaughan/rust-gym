use std::fmt;

fn main() {
    let a = Item(1, "stick", "Butter");
    let b = Item(1, "dozen", "eggs");
    let c = Item(3, "whole", "Granny Smith apples");
    let g = GroceryList {items: Vec::from([a, b, c])};
    println!("{}", g)
}

struct Item(u8, &'static str, &'static str);

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

struct GroceryList {
    items: Vec<Item>,
}


impl GroceryList {
    fn count(&self) -> u8 {
        return self.items.iter().map(|n| n.0).sum::<u8>()
    }
}

impl fmt::Display for GroceryList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Things to buy:\n")?;
        for item in &self.items {
            write!(f, "{}\n", item)?;
        }
        write!(f, "({} items total)", &self.count())
    }
}