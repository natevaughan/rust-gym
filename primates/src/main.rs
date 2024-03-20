
struct Chimpanzee {
    name: String
}

trait Animal {
    fn name(&self) -> &str;
    fn speak(&self);
}

impl Animal for Chimpanzee {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) {
        println!("A Chimpanzee named {} says Ooooo oooo eeeeee eeeee!!!", &self.name)
    }
}

fn main() {
    let c = Chimpanzee{name: "Lola".to_string()};
    c.speak();
}