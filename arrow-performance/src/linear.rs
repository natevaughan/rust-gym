
pub fn interpolate_linear(first: f64, second: f64, scalar: f64) -> f64 {
    let calc = (second - first) * scalar + first;
    println!("Linearly interpolated {} and {}, got {}", first, second, calc);
    calc
}

pub fn scale(first: f64, second: f64, val: f64) -> f64 {
    let calc = (val - first) / (second - first);
    println!("scaled {} and {}, got {}", first, second, calc);
    calc
}

pub fn lin(a: f64, b: f64, x: f64) -> f64 {
    a * x + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_in_range() {
        assert_eq!(scale(5.0, 10.0, 7.5), 0.5);
    }

    #[test]
    fn test_scale_out_of_range() {
        assert_eq!(scale(5.0, 10.0, 2.5), -0.5);
    }
}