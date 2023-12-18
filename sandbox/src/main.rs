fn main() {
    println!("Don't forget to add semicolons to anything that is not a return statement in Rust!");
    let mut a = 10;
    println!("Let's do some addition. Let a = {a}");
    let b = a;
    a = 11;
    println!("Now let b = a and set a = 11");
    println!("b = {b}");
    println!("a = {a}");
    let mut c = Container { val: a };
    println!("Now wrap it in a container called Container and assign to c: ");
    let d = c;
    c.val = 12;
    println!("Just for fun we will set d = c and change c's value");
    println!("c.val = {}", c.val);
    println!("d.val = {}", d.val);   
}

struct Container {
    val: i64
}

impl Copy for Container { }

impl Clone for Container {
    fn clone(&self) -> Container {
        *self
    }
}