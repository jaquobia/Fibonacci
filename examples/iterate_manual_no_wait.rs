use std::error::Error;
use fibonacci::fibonacci_at_loop;

fn main() -> Result<(), Box<dyn Error>> {
    for n in 0..25 {
        let fib = fibonacci_at_loop::<u16>(n).ok_or_else(|| "Ran out of memory for integer size!")?;
        println!("{}: {}", n, fib);
    }
    return Ok(())
}
