fn main() {
    let mut a = 10;
    let ref mut b = a;
    *b += 1;
    // does not compile due to mutable borrow
    println!("{b} {a}")
}