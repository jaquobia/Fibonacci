use std::error::Error;
use fibonacci::fibonacci;

fn main() -> Result<(), Box<dyn Error>> {
    for (n, fib) in fibonacci::<u64>().enumerate() {
        println!("{}: {}", n, fib);
    }

    println!("Ran out of memory for integer size!");
    return Ok(())
}
