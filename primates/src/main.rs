trait Animal {
    fn name(&self) -> String;
    fn speak(&self) -> String;
}

struct Chimpanzee {
    name: String
}

struct Jellyfish;

impl Animal for Jellyfish {
    fn name(&self) -> String {
        "Jellyfishes don't have names, silly".to_string()
    }

    fn speak(&self) -> String  {
        "Jellyfishes don't make sounds, silly".to_string()
    }
}
impl Animal for Chimpanzee {
    fn name(&self) -> String {
        self.name.to_string()
    }

    fn speak(&self) -> String {
        format!("A Chimpanzee named {} says Ooooo oooo eeeeee eeeee!!!", &self.name)
    }
}

fn main() {
    let c = Chimpanzee{name: "Lola".to_string()};
    let j = Jellyfish{};
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(c), Box::new(j)];   
    let_the_animals_speak(animals) 
}

fn let_the_animals_speak(v: Vec<Box<dyn Animal>>) {
    for a in v.into_iter() {
        println!("{}", a.speak())
    }
}