use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("original arguments: {}", args.join(" "));
    let words: Vec<String> = args[1..].iter().map(|x| x.to_string()).collect();
    println!("word arguments: {}", words.join(" "));
    let reversed = reverse(&words);
    println!("reversed arguments: {}", reversed.join(" "));
    println!("and finally let's check the original arguments: {}", args.join(" "));
}


fn reverse(strings: &Vec<String>) -> Vec<String> {
    let mut newvec = strings.to_vec();
    newvec.reverse();
    newvec
}