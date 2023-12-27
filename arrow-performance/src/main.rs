use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run <outside_air_temp> <pressure_alt>");
        return
    }
    let iso_temp = &args[1];
    let pressure_alt: f64 = args[2].parse().expect("Please provide a numeric value for pressure altitude");
    let pressure_alt_scalar = scale(0.0, 7000.0, pressure_alt);
    let a = interpolate( 0.01, 0.03, pressure_alt_scalar);
    println!("Pressure alt scalar: {}, a: {}", pressure_alt_scalar, a);
    println!("Takeoff ground roll: {} {}", iso_temp, pressure_alt)
}

fn interpolate(first: f64, second: f64, scalar: f64) -> f64 {
    println!("Interpolating between {} and {} for scalar {}", first, second, scalar);
    let calc = (second - first) * scalar + first;
    println!("Got {}", calc);
    return calc
}

fn scale(first: f64, second: f64, val: f64) -> f64 {
    return (val - first) / (second - first);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
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