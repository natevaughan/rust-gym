fn main() {
    println!("Test 123")
}

/**
 * Matrix must be square
 */
fn rotate_matrix_left(m: Vec<Vec<i32>>) -> Result<Vec<Vec<i32>>, String> {
    let len = m.len();
    for row in &m {
        if row.len() != len {
            return Err("Bad bad not good".to_string())
        }
    }
    let mut rotated: Vec<Vec<i32>> = vec![];
    for i in 0..len {
        rotated.push(vec![]);
        for j in 0..len {
            rotated[i].push(m[j][len - i - 1])
        }
    }
    Ok(rotated)
}

#[cfg(test)]
mod tests {
    use crate::rotate_matrix_left;

    #[test]
    fn test_return_err_if_not_square_1() {
        let bad = vec![vec![1i32, 2i32], vec![3i32]];
        let res = rotate_matrix_left(bad);
        assert!(res.is_err())
    }

    #[test]
    fn test_return_err_if_not_square_2() {
        let m = vec![vec![1i32], vec![2i32]];
        let res = rotate_matrix_left(m);
        assert!(res.is_err());
    }

    #[test]
    fn test_same_matrix_if_one_element() {
        let one = vec![vec![1i32]];
        let res = rotate_matrix_left(one);
        assert!(res.is_ok());
        assert_eq!(1i32, res.unwrap()[0][0]);
    }

    #[test]
    fn test_2_x_2() {
        let two = vec![vec![1i32, 2i32], vec![3i32, 4i32]];
        let res = rotate_matrix_left(two);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(2i32, res[0][0]);
        assert_eq!(4i32, res[0][1]);
        assert_eq!(1i32, res[1][0]);
        assert_eq!(3i32, res[1][1]);
    }

    #[test]
    fn test_3_x_3() {
        let three = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let res = rotate_matrix_left(three);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(3i32, res[0][0]);
        assert_eq!(5i32, res[1][1]);
        assert_eq!(4i32, res[2][1]);
        assert_eq!(7i32, res[2][2]);
    }

    #[test]
    fn test_2_x_2_returns_to_original_after_four_rotations() {
        let two = vec![vec![1i32, 2i32], vec![3i32, 4i32]];
        let res = rotate_matrix_left(rotate_matrix_left(rotate_matrix_left(rotate_matrix_left(two).unwrap()).unwrap()).unwrap()).unwrap();
        assert_eq!(1i32, res[0][0]);
        assert_eq!(2i32, res[0][1]);
        assert_eq!(3i32, res[1][0]);
        assert_eq!(4i32, res[1][1]);
    }
}