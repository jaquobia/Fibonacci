use std::collections::HashMap;
use num_traits::{One, CheckedAdd, Zero};

pub trait FibonacciAcceptableType: CheckedAdd + Clone + One + Zero {}
impl <T: CheckedAdd + Clone + One + Zero> FibonacciAcceptableType for T {}

pub fn fibonacci_construct_memo<I: FibonacciAcceptableType>() -> HashMap<usize, I> {
    let mut memo = HashMap::default();
    memo.insert(0, I::zero());
    memo.insert(1, I::one());
    memo
}

pub struct FibonacciIterator<I: FibonacciAcceptableType> {
    memo: HashMap<usize, I>,
    current: usize,
}

impl<I: FibonacciAcceptableType> FibonacciIterator<I> {
    pub fn new() -> Self { 
        Self { memo: fibonacci_construct_memo(), current: 0 } 
    }
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

pub fn fibonacci<I: FibonacciAcceptableType>() -> FibonacciIterator<I> {
    FibonacciIterator::new()
}

pub fn fibonacci_at<I: FibonacciAcceptableType>(n: usize) -> Option<I> {
    let mut memo = fibonacci_construct_memo();
    fibonacci_at_memo(n, &mut memo);
    memo.get(&n).cloned()
}

pub fn fibonacci_at_loop<I: FibonacciAcceptableType>(n: usize) -> Option<I> {
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

pub fn fibonacci_at_memo<I: FibonacciAcceptableType>(current_fib: usize, fib_memo: &mut HashMap<usize, I>) -> Option<I> {
    let fib = fib_memo.get(&current_fib).cloned();
    return fib.or_else(|| {
        let fibn1 = fibonacci_at_memo(current_fib-1, fib_memo).clone();
        let fibn2 = fibonacci_at_memo(current_fib-2, fib_memo).clone();
        fibn1.zip(fibn2)
            .and_then(|(a, b)| a.checked_add(&b) )
            .map(|fib| {
                fib_memo.insert(current_fib, fib.clone());
                fib
            })
    });
}
