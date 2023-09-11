use std::collections::HashMap;
use num_traits::{One, CheckedAdd, Zero};

/// The super trait that dictates valid integer types to use with the fibonacci functions.
pub trait FibonacciAcceptableType: CheckedAdd + Clone + One + Zero {}
impl <T: Clone + CheckedAdd + One + Zero> FibonacciAcceptableType for T {}

/// Construct and return a memoization hashmap that has n=0 and n=1 filled by default
pub fn fibonacci_construct_memo<I: FibonacciAcceptableType>() -> HashMap<usize, I> {
    let mut memo = HashMap::default();
    memo.insert(0, I::zero());
    memo.insert(1, I::one());
    memo
}

/// A Fibonacci iterator that stores a memoization of the values it has previously iterated over.  
/// Example:
/// ```
/// use fibonacci::FibonacciIterator;
/// fn main() {
///     let fib_iter: FibonacciIterator<u16> = FibonacciIterator::new();
///     for (n, fib) in fib_iter.enumerate() {
///          println!("{}: {}", n, fib);
///     }
/// }
/// ```
pub struct FibonacciIterator<I: FibonacciAcceptableType> {
    memo: HashMap<usize, I>,
    current: usize,
}

impl<I: FibonacciAcceptableType> FibonacciIterator<I> {
    /// Constructs a new Fibonacci iterator starting from n=0
    pub fn new() -> Self { 
        Self { memo: fibonacci_construct_memo(), current: 0 } 
    }

    /// Constructs a new Fibonacci iterator starting from n  
    /// Will contain values of Fibonacci(n) from 0..n
    pub fn new_at(n: usize) -> Self {
        let mut memo = fibonacci_construct_memo();
        fibonacci_at_memo(n, &mut memo);
        Self { memo, current: n }
    }

    /// Takes self and returns its memoization of values
    pub fn into_memo(self) -> HashMap<usize, I> { self.memo }
}

impl<I: FibonacciAcceptableType> Iterator for FibonacciIterator<I> {
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        let fib = fibonacci_at_memo(self.current, &mut self.memo);
        self.current += 1;
        fib
    }
}

/// Returns a Fibonacci iterator starting from n  
/// Will contain values of Fibonacci(n) from 0..n
///
/// # Examples
/// ```
/// use fibonacci::fibonacci;
/// 
/// for (n, fib) in fibonacci_n::<u16>(6).enumerate() {
///     println!("{}: {}", n, fib);
/// }
///```
pub fn fibonacci_n<I: FibonacciAcceptableType>(n: usize) -> FibonacciIterator<I> {
    FibonacciIterator::new_at(n)
}

/// Returns a Fibonacci iterator starting from 0  
///
/// # Examples
/// ```
/// use fibonacci::fibonacci;
/// 
/// for (n, fib) in fibonacci::<u16>().enumerate() {
///     println!("{}: {}", n, fib);
/// }
///```
pub fn fibonacci<I: FibonacciAcceptableType>() -> FibonacciIterator<I> {
    FibonacciIterator::new()
}

/// Calculates Fibonacci(n), returning `Option::None` if the value overflows
///
/// # Examples
///
/// ```
/// use fibonacci::fibonacci_at;
///
/// assert_eq!(fibonacci_at(6), 5);
/// ```
pub fn fibonacci_at<I: FibonacciAcceptableType>(n: usize) -> Option<I> {
    let mut memo = fibonacci_construct_memo();
    fibonacci_at_memo(n, &mut memo);
    memo.get(&n).cloned()
}

/// Calculates Fibonacci(n), returning `Option::None` if the value overflows
/// This method doesn't use a lookup table and computes Fibonacci(n) through a bottom-up approach
/// # Examples
///
/// ```
/// use fibonacci::fibonacci_at_fast;
///
/// assert_eq!(fibonacci_at_fast(6), 5);
/// ```
pub fn fibonacci_at_fast<I: FibonacciAcceptableType>(n: usize) -> Option<I> {
    let mut a = I::zero();
    let mut b = I::one();
    if n == 0 { return Some(a); }
    for _ in 2..n {
       let c = b.checked_add(&a)?; 
       a = b;
       b = c;
    }
    return Some(b);
}

/// Returns the value of Fibonacci(n) either through lookup of the memo or by calculating all
/// unknown values of Fibonacci(2..=n) in a top-down approach.
/// Assumes that for any value already in the memo, memo\[k\] = Fibonacci(k).
/// Intended to be used in an iterative fashion where the same memo is continusouly passed in, or a
/// memoized top-down single use approach.
pub fn fibonacci_at_memo<I: FibonacciAcceptableType>(n: usize, memo: &mut HashMap<usize, I>) -> Option<I> {
    let fib = memo.get(&n).cloned();
    return fib.or_else(|| {
        let fibn1 = fibonacci_at_memo(n-1, memo).clone();
        let fibn2 = fibonacci_at_memo(n-2, memo).clone();
        fibn1.zip(fibn2)
            .and_then(|(a, b)| a.checked_add(&b) )
            .map(|fib| {
                memo.insert(n, fib.clone());
                fib
            })
    });
}
