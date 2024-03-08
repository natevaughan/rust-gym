fn main() {
	let maybe = std::option::Option::Some("hello\n");
	maybe.map(print_if_some);

	let my_arr: [&str; 3] = ["hello", ", ", "world"];
	my_arr.into_iter().map(print_if_some).collect::<()>();

	let a: [i32; 3] = [5, 6, 7];
	let b: [i32; 3] = [10, 5, 4];
	let result = compare_triplets(a, b);

	println!("{:?}", result);
}

fn print_if_some(s: &str) {
	print!("{}", s);
}


fn compare_triplets(a: [i32; 3], b: [i32; 3]) -> Vec<i32> {
	let mut v = vec![0, 0];
	for i in 0..3 {
		if a[i] > b[i] {
			v[0] += 1
		} else if a[i] < b[i] {
			v[1] += 1
		}
	}
	v
}
