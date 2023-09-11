use std::error::Error;
use fibonacci::fibonacci_at_fast;

fn main() -> Result<(), Box<dyn Error>> {
    let term = console::Term::stdout();
    for n in 0..27 {
        term.read_char()?;
        let fib = fibonacci_at_fast::<u16>(n).ok_or_else(|| "Ran out of memory for integer size!")?;
        println!("{}: {}", n, fib);
    }
    return Ok(())
}
