fn main() {
	print_min_max_sum(&[5, 6, 7, 8, 9])
}

fn print_min_max_sum(arr: &[i32]) {
	let (min, max) = min_max_sum(arr);
	println!("min: {}, max: {}", min, max)
}

fn min_max_sum(arr: &[i32]) -> (i32, i32) {
	let mut min = arr.iter().min().unwrap();
	let mut max = arr.iter().max().unwrap();
	let sum: i32 = arr.iter().sum();
	(sum - max, sum - min)
}

#[cfg(test)]
mod tests {
	use crate::min_max_sum;

	#[test]
	fn test_min_max_sum_1() {
		let test_arr: [i32; 5]  = [1, 2, 3, 4, 5];
		let (min, max) = min_max_sum(&test_arr);
		assert_eq!(10i32, min);
		assert_eq!(14i32, max);
	}
}
