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
	println!("Type of foo: {}", get_type_name(&foo))
}

// source https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable
fn get_type_name<T>(_: &T) -> String {
	format!("{}", std::any::type_name::<T>())
}
