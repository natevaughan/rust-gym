use std::fmt;

fn main() {
    let b737_800 = Aircraft {
        manufacturer: "Boeing".to_string(),
        model_shorthand: "B73M".to_string(),
        model_name: "737".to_string(),
        model_variant: "MAX8".to_string()
    };

    let be36 = Aircraft {
        manufacturer: "Beechcraft".to_string(),
        model_shorthand: "BE36".to_string(),
        model_name: "Bonanza".to_string(),
        model_variant: "A36".to_string()
    };
    println!("This is a {0}, but if ATC asks for type, say {0:?}", b737_800);

    let hangar = Hangar(Vec::from([b737_800, be36]));
    println!("{}", hangar)
}

struct Aircraft {
    manufacturer: String,
    model_shorthand: String,
    model_name: String,
    model_variant: String
}

impl fmt::Display for Aircraft {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.manufacturer, self.model_name, self.model_variant)
    }
}

impl fmt::Debug for Aircraft {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.model_shorthand)
    }
}

struct Hangar(Vec<Aircraft>);

impl fmt::Display for Hangar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        let mut i = 0;
        for a in vec {
            write!(f, "{:?}", a)?;
            i += 1;
            if i != vec.len() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}