fn main() {
	plus_minus(&[-2, 0, 3, 3]);
	staircase(1);
	staircase(2);
	staircase(10);
}

fn staircase(n: i32) {
	const HASH: &str = "#";
	const SPACE: &str = " ";
	for i in 1..=n {
		let mut line = "".to_owned();
		for j in 1..=(n - i) {
			line.push_str(SPACE);	
		}
		for j in (n - i + 1)..=n { 
			line.push_str(HASH);
		}
		println!("{}", line)
	}
}

struct Counts {
	positive: u32,
	negative: u32,
	zero: u32,
}

impl Counts {
	fn print_ratio(&self) {
		let total = (self.positive + self.negative + self.zero) as f64;
		println!("{:.6}", (self.positive as f64) / total);
		println!("{:.6}", (self.negative as f64) / total);
		println!("{:.6}", (self.zero as f64) / total);
	}	

	fn new() -> Counts {
		Counts{positive: 0, negative: 0, zero: 0}
	}
}


fn plus_minus(arr: &[i32]) {
	let mut counts = Counts::new();
	for el in arr {
		if *el > 0 {
			counts.positive += 1
		} else if *el < 0 {
			counts.negative += 1
		} else {
			counts.zero += 1
		}
	}
	counts.print_ratio();
}
