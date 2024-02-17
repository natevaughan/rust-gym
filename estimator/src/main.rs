use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short, long)]
    size: i32,

    #[arg(short, long, default_value_t = true)]
    modern: bool,
}

fn main() {
    let args = Args::parse();
    println!("estimating using modern {} and size {}:", args.modern, args.size);
    let est = estimate(args.size, args.modern);
    println!("{}", est)
}

fn estimate(size: i32, modern: bool) -> f64 {
    if modern {
        2.42 *  f64::from(size).ln() + 5.0668
    } else {
        2.42 * f64::from(size).ln() + 4.5668
    }
}