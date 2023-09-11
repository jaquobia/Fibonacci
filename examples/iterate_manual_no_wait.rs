use std::error::Error;
use fibonacci::fibonacci_at_fast;

fn main() -> Result<(), Box<dyn Error>> {
    for n in 0..27 {
        let fib = fibonacci_at_fast::<u16>(n).ok_or_else(|| "Ran out of memory for integer size!")?;
        println!("{}: {}", n, fib);
    }
    return Ok(())
}
