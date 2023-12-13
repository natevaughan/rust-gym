use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut mut_args = args.to_owned();
    let args_to_sort = &mut mut_args[1..];
    if args_to_sort.len() == 0 {
        panic!("Nothing to sort")
    }
    args_to_sort.sort();
    println!("Sorted {} {}s: {}", args_to_sort.len(), extract_type(&args_to_sort[0]), args_to_sort.join(" "));
}

// source https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable/58119924#58119924
fn extract_type<T>(_: &T) -> &'static str {
    let long_type_name = std::any::type_name::<T>();
    return long_type_name.split("::").last().unwrap_or(long_type_name);
}