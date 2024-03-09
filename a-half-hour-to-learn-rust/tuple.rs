fn main() {
    let a = (10, 11);
    // std is a crate (package) and cmp is a module (source file) and min is a function
    use std::cmp::{min, max}; // import min & max function; alternative to import everything: 'use std::cmp::*;'
    println!("the smaller number is: {}", min(a.0, a.1));
    println!("the bigger number is: {}", max(a.0, a.1));
    let text = "Hello World!";
    println!("length of string is: {}", text.len()); // alternative: 'str::len(text)'
    // 'str' is know as a primitive type
}