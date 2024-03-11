fn main() {
	print_min_max_sum(&[5, 6, 7, 8, 9])
}

fn print_min_max_sum(arr: &[i32]) {
	let (min, max) = min_max_sum(arr);
	println!("min: {}, max: {}", min, max)
}

fn min_max_sum(arr: &[i32]) -> (i64, i64) {
	let first: i64 = arr[0].into();
	let mut min = first;
	let mut max = first;
	let mut sum = first;
	for i in 1..arr.len() {
		let el: i64 = arr[i].into();
		sum += el;
		if el < min {
			min = el
		}
		if el > max {
			max = el
		}
	}
	(sum - max, sum - min)
}

#[cfg(test)]
mod tests {
	use crate::min_max_sum;

	#[test]
	fn test_min_max_sum_1() {
		let test_arr: [i32; 5]  = [1, 2, 3, 4, 5];
		let (min, max) = min_max_sum(&test_arr);
		assert_eq!(10i64, min);
		assert_eq!(14i64, max);
	}
}
