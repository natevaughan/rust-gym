pub fn quad(a: f64, b: f64, c: f64, x: f64) -> f64 {
    b * (x - a).powi(2) + c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quad() {
        assert_eq!(quad(-4044.0, 0.0001636, -7550.0, 2750.0), 1.5201295999995637);
    }
}