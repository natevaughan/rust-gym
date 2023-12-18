fn main() {
    fn double(num: u128) -> u128 {
      num * 2
    }
  
    let int: i32 = 32;
    let big_int = 10;
    let float = 1.2;
  
    let outcome = double(big_int);
    let int2 = double(int.try_into().unwrap());
    let doubled_outcome = outcome;
  
    println!("{doubled_outcome}");
  
    println!("{int2} {float}");

    println!("Let's try a statement");

    let foo = {
        let bar = f64::from(doubled_outcome as u32) * float;
        println!("({doubled_outcome} * {float}) + {int2} = ");
        bar as u64 + u64::try_from(int2).unwrap_or(0)
    };

    println!("{foo}")
    
  }
  