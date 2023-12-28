
pub fn interpolate_geometric(first: f64, second: f64, scalar: f64) -> f64 {
    let calc = (first / second).powf(1.0 - scalar) * second;
    println!("Geometrically interpolated {} and {}, got {}", first, second, calc);
    calc
}

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

    #[test]
    fn test_interpolate_geometric_1() {
        assert_eq!(interpolate_geometric(2.0, 8.0, 0.5), 4.0);
    }

    #[test]
    fn test_interpolate_geometric_2() {
        assert_eq!(interpolate_geometric(2.0, 32.0, 0.25), 4.0);
    }

    #[test]
    fn test_interpolate_geometric_3() {
        assert_eq!(approx(interpolate_geometric(3.0, 81.0, 1.0 / 3.0), 6), approx(9.1, 6));
    }

    fn approx(number: f64, digits: i32) -> isize {
        (number * (10.0_f64).powi(digits) + 0.5) as isize
    }
}

// geometric mean
// ((first / second) ^ ratio) * first