// use std::prelude::v1::*; //this is default import
fn main() {
    let _v: Vec<i32> = Vec::new(); // this is a immutable array, for values of the same type
    let mut v2 = Vec::new(); // this is a dynamic array, for values of the same type
    v2.push(5);
    v2.push(6);
    v2.push(7);
    // v2.push("7"); // this will cause an error, as v2 must be of a single type
    println!("{:?}", v2);
}