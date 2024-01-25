use std::fmt;

fn main() {
    let ape = 0b111101;
    println!("The chimpanzee is {} years old", ape);
    let orang = 0x3b3;
    println!("The orangutan has {} sticks in his collection", orang);
    println!("The matrix is\n{}", Matrix(1.5, 2.5, 3.5, 4.5))
}

struct Matrix(f64, f64, f64, f64);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Matrix(tl, tr, bl, br) = self;
        write!(f, "({}, {})\n({}, {})", tl, tr, bl, br)
    }
}

fn transpose(m: Matrix) -> Matrix {
    
}