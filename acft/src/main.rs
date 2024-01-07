use std::fmt;

fn main() {
    let b737_800 = Aircraft {
        manufacturer: "Boeing".to_string(),
        model_shorthand: "B73M".to_string(),
        model_name: "737".to_string(),
        model_variant: "MAX8".to_string()
    };
    println!("This is a {0}, but if ATC asks for type, say {0:?}", b737_800)
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