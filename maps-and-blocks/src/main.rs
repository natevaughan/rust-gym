fn main() {
	let maybe = std::option::Option::Some("hello\n");
	maybe.map(print_if_some);
//	let my_vec = vec!["hello", ", ", "world"];
//	my_vec.iter().map(print_if_some);
	let my_arr: [&str; 3] = ["hello", ", ", "world"];

	my_arr.into_iter().map(print_if_some).collect::<()>();
}

fn print_if_some(s: &str) {
	print!("{}", s);
}
