// Converting any type to a `String` is as simple as implementing
// the `ToString` trait. However, it is preferred to instead
// implement the `fmt::Display` trait, which automatically
// provides a `ToString` implementation

use std::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius: {}", self.radius)
    }
}

// Parsing a string
// The FromStr trait requires a function `parse` that converts from
// a string to the type. This is implemented for many types within
// the std lib

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // Note: in the real world we probably would not want to
    // use .unwrap() here - that will panic on a parse error
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
