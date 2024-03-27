fn main() {
	println!("{}", caesar(3, "xyzXYZ"))
}

fn caesar(i: u8, text: &str) -> String {
	text.chars().map(|c| rotate_char(c, i)).collect()
}

fn rotate_char(c: char, i: u8) -> char {
	let c_num = c as u8;
	if c_num >= 65 && c_num <= 90 {
		((c_num - 65 + i) % 26 + 65) as char
	} else if c_num >= 97 && c_num <= 122 {
		((c_num + i - 97) % 26 + 97) as char
	} else {
		c
	}
}
