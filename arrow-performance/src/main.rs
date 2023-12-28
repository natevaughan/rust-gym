use std::env;
use crate::linear::scale;
use crate::linear::interpolate_linear;
use crate::linear::lin;
use crate::quadratic::quad;
use crate::quadratic::interpolate_geometric;

mod linear;
mod quadratic;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run <outside_air_temp> <pressure_alt> <take_off_weight>");
        return
    }
    let adj_curve_msl = QuadCurve {
        name: String::from("Flaps 0 ground roll (y) by ISO Temp F (x) 0ft MSL"),
        scalar: 0.0,
        a: -202.0, 
        b: 0.023,
        c: 180.0
    };
    let seven_k_msl = QuadCurve {
        name: String::from("Flaps 0 ground roll (y) from ISO Temp F (x) 7000ft MSL"),
        scalar: 7000.0,
        a: -264.0, 
        b: 0.0331,
        c: 360.0
    };
    let iso_temp_c: f64 = args[1].parse().expect("Please provide a numeric value for ISO temp");
    let iso_temp_f = lin(1.8, 32.0, iso_temp_c);
    let pressure_alt: f64 = args[2].parse().expect("Please provide a numeric value for pressure altitude");
    let take_off_weight: f64 = args[3].parse().expect("Please provide a numeric value for takeoff weight");
    // First calc
    let init_roll = calc_from_curve(&scale_curve(adj_curve_msl, seven_k_msl, pressure_alt), iso_temp_f);


    let thirty_three_hundred =  QuadCurve {
        name: String::from("Flaps 0 ground roll (y) adjustment curve by gross weight (x) 3300 initial ground roll"),
        scalar: 3300.0,
        a: -4044.0, 
        b: 0.0001636,
        c: 4508.492
    };
    
    let seventeen_ten =  QuadCurve {
        name: String::from("Flaps 0 ground roll (y) adjustment curve by gross weight (x) 1710 initial ground roll"),
        scalar: 1710.0,
        a: -1115.0,
        b: 0.0001587,
        c: 4647.696
    };
    println!("Init roll: {:>5}", init_roll.round() as i64);

    let adjustment = calc_from_curve(&scale_curve(seventeen_ten, thirty_three_hundred, init_roll), take_off_weight);

    let final_roll = init_roll + adjustment;
    let weight_scalar = scale(2000.0, 2750.0, take_off_weight);
    let vr = interpolate_linear(60.0, 71.0, weight_scalar);
    println!("Ground roll: {:>5}", final_roll.round() as i64);
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

/*

    let a1 = interpolate(-1115.0, -4044.0, init_roll_scalar);
    let b1 = interpolate(0.0001587, 0.0001636, init_roll_scalar);
    let c1 = quad(a1, b1, 0.0, 2750.0);
    let adjustment = quad(a1, b1, -c1, take_off_weight);
     */
struct QuadCurve {
    name: String,
    scalar: f64,
    a: f64,
    b: f64,
    c: f64,
}