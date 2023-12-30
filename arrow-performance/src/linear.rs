
pub fn interpolate_linear(first: f64, second: f64, scalar: f64) -> f64 {
    (second - first) * scalar + first
}

pub fn scale(first: f64, second: f64, val: f64) -> f64 {
    (val - first) / (second - first)
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