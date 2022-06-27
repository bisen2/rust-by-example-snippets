// The From and Into traits are used to provide conversions between
// non-primitive types

// The From trait allows a type to define how to create itself
// from another type

#![allow(dead_code)]
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// The Into trait is the reciprocal of the From trait. If you have
// implemented the From trait on your type, Into will call it
// when necessar

fn main() {
    // using a `from` function provided by the std lib
    let my_str = "hello";
    let _my_string = String::from(my_str);

    // using our custom implementation of the From trait
    let num = Number::from(30);
    println!("My number is: {:?}", num);

    // using the implicit definition of into we get from having
    // defined the From trait
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
