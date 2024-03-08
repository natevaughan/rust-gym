fn main() {
	let foo = vec![Cat{name: "Mango".to_string(), age: 11}];
	println!("{:?}", foo);
	for i in 0..5 {
		let el = foo.get(i);
		match el {
			None => println!("No element at {}", i),
			Some(c) => println!("{} is {}", c.name, c.age),
		}
	}
}

#[derive(Debug)]
struct Cat {
	name: String,
	age: u32,
}
