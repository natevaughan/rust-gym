fn main() {
	println!("6: {:?}", factors(6));
	println!("8: {:?}", factors(8));
	println!("48: {:?}", factors(48));
}

fn factors(m: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = Vec::new();
    let mut i = 2i32;
    let mut tower = m;
    loop {
        if i == m {
            break;
        }
        if tower % i == 0i32 {
            factors.push(i);
            tower = tower / i; 
        } else {
            i += 1;
        }
    }
	factors
}
