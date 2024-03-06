use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
	#[arg(short, long)]
	greeting: String
}

fn main() {
	let args = Args::parse();
	println!("{}, world!", args.greeting);
	let foo;
	foo = 8;
	println!("Type of foo: {}", get_type_name(&foo));
	let (x, y) = here_is_some_stuff();
	println!("Type of x: {}, type of y: {}", get_type_name(&x), get_type_name(&y));

	// some variable shadowing
	let a = 42.0f64;
	println!("Type of a: {}", get_type_name(&a));
	for i in 2 .. 4 {
		let a = i;
		println!("Type of a in loop is: {} : {}", a, get_type_name(&a));
	}
	println!("Type of a after loop: {}", get_type_name(&a));
	
}

fn here_is_some_stuff() -> (i32, f64) {
	(20, 50.5)
}

// source https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable
fn get_type_name<T>(_: &T) -> String {
	format!("{}", std::any::type_name::<T>())
}
