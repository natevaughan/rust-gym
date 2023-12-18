fn main() {
    use Bing::FOO;

    let scoped = "is this scope shadowed!?".to_string();
    {
        let scoped = "YES!".to_string();
        println!("{}", scoped
    )
    }
    println!("{}", scoped);
    println!("{}", FOO)
}

mod Bing {
    pub static FOO: &str = "exported value!!";
}