fn main() {
    let mut a = Foo { bar: 10 };
    let ref mut b = a;
    b.bar += 1;
    // does not compile due to immutable borrow
    println!("{} {}", b.bar, a.bar)
}

struct Foo {
    bar: i64
}