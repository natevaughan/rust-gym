use std::env;
use crate::linear::scale;
use crate::linear::interpolate_linear;
use crate::linear::lin;
use crate::quadratic::quad;

mod linear;
mod quadratic;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run <outside_air_temp> <pressure_alt> <take_off_weight>");
        return
    }
    let iso_temp_c: f64 = args[1].parse().expect("Please provide a numeric value for ISO temp");
    let iso_temp_f = lin(1.8, 32.0, iso_temp_c);
    let pressure_alt: f64 = args[2].parse().expect("Please provide a numeric value for pressure altitude");
    let take_off_weight: f64 = args[3].parse().expect("Please provide a numeric value for takeoff weight");

    // First calc
    let curves = [(0, QuadCurve {
        name: String::from("Flaps 0 ground roll (y) by ISO Temp F (x) 0ft MSL"),
        scalar: 0.0,
        a: -184.484, 
        b: 0.024538,
        c: 283.3
    }), (7000, QuadCurve {
        name: String::from("Flaps 0 ground roll (y) from ISO Temp F (x) 7000ft MSL"),
        scalar: 7000.0,
        a: -175.482, 
        b: 0.051501,
        c: 1064.07
    })];

    let nearest_low = &curves[0].1;
    let nearest_high = &curves[1].1;
    let val1 = calc_from_curve(nearest_low, iso_temp_f);
    let val2 = calc_from_curve(nearest_high, iso_temp_f);
    let init_roll = interpolate_linear(val1, val2, scale(nearest_low.scalar, nearest_high.scalar, pressure_alt));
    let weight_scalar = scale(2000.0, 2750.0, take_off_weight);
    let vr = interpolate_linear(60.0, 71.0, weight_scalar);
    println!("Ground roll: {:>5}", init_roll.round() as i64);
    println!("Vr:          {:>5}", vr.round() as i64);
}

fn calc_from_curve(c: &QuadCurve, x: f64) -> f64 {
    quad(c.a, c.b, c.c, x)
}

fn scale_curve(c1: QuadCurve, c2: QuadCurve, s: f64) -> QuadCurve {
    let scalar = scale(c1.scalar, c2.scalar, s);
    QuadCurve {
        name: String::from("Interpolated curve"),
        scalar: s,
        a: interpolate_linear(c1.a, c2.a, scalar),
        b: interpolate_linear(c1.b, c2.b, scalar),
        c: interpolate_linear(c1.c, c2.c, scalar),
    }
}

struct QuadCurve {
    name: String,
    scalar: f64,
    a: f64,
    b: f64,
    c: f64,
}