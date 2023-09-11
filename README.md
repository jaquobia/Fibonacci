# Fibonacci
This is just my example at creating a fibonacci crate.  
  
It is flexible in that it allows usage of any integer type that is  
`numtraits::One + numtraits::Zero + Clone`,  
along with the fact that there is an iterator implementation  
`fibonacci::fibonacci()`  
and other functions for different use cases.

# Examples
Some examples are available, and can be run with `cargo run --release --example=<EXAMPLE>`.  
The examples show off some ways the crate can be used and the limits of different unsigned integer types.
