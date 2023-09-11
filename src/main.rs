use std::{collections::HashMap, error::Error};
use num_traits::{One, CheckedAdd};

type IntType = num_bigint::BigUint;

fn main() -> Result<(), Box<dyn Error>> {
    let term = console::Term::stdout();
    let mut fib_memo: HashMap<usize, IntType> = HashMap::default();
    let mut current_fib = 0;
    fib_memo.insert(0, IntType::one());
    fib_memo.insert(1, IntType::one());

    loop {
        term.read_char()?;
        let fib = fibonacci(current_fib, &mut fib_memo).ok_or_else(|| format!("Ran out of memory for integer size by {current_fib}!"))?;
        println!("{}: {}", current_fib, fib);
        current_fib+=1;
    }
}

fn fibonacci<I: CheckedAdd + Clone + One>(current_fib: usize, fib_memo: &mut HashMap<usize, I>) -> Option<I> {
    let fib = fib_memo.get(&current_fib).cloned();
    return fib.or_else(|| {
        let fibn1 = fibonacci(current_fib-1, fib_memo).clone();
        let fibn2 = fibonacci(current_fib-2, fib_memo).clone();
        fibn1.zip(fibn2)
            .and_then(|(a, b)| a.checked_add(&b) )
            .map(|fib| {
                fib_memo.insert(current_fib, fib.clone());
                fib
            })
    });
}
