use std::error::Error;
use fibonacci::fibonacci;
use num_bigint::BigUint;

fn main() -> Result<(), Box<dyn Error>> {
    let term = console::Term::stdout();
    for (n, fib) in fibonacci::<BigUint>().enumerate() {
        term.read_char()?;
        println!("{}: {}", n, fib);
    }

    println!("Ran out of memory for integer size!");
    return Ok(())
}
