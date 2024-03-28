fn main() {
	print_fwd_rev();
}

fn print_fwd_rev() {
	let v: Vec<i32> = vec![1,2,3,4,5];
	for i in &v {
		println!("{}", i)
	}
	for i in 1..=v.len() {
		println!("{}", v[v.len() - i])
	}
	v.clone().into_iter().for_each(|i| println!("{}", i));
	v.into_iter().rev().for_each(|i| println!("{}", i));
}

fn n_chars(c: char) -> String {
	let uc = c as u8;
	let mut n = 0;
	if uc as i32 - 97 >= 0i32 {
		n = uc - 97
	} else if uc as i32 - 65 >= 0i32 {
		n = uc - 65
	}
	let mut concat = c.to_string();
	for _ in 0..n {
		concat.push(c)
	}
	concat
}

#[cfg(test)]
mod tests {
	use crate::n_chars;

	#[test]
	fn test_n_chars_1() {
		let v = n_chars('c');
		assert_eq!("ccc".to_string(), v)
	}
	
	#[test]
	fn test_n_chars_2() {
		let v = n_chars('C');
		assert_eq!("CCC".to_string(), v)
	}
}
