fn main() {
	let foo = vec![Cat{name: "Mango".to_string(), age: 11}];
	println!("{:?}", foo);
	for i in 0..5 {
		foo.get(i).map(|c| println!("{} is {}", c.name, c.age));
	}
}

#[derive(Debug)]
struct Cat {
	name: String,
	age: u32,
}
