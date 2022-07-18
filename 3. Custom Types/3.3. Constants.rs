/*
    Rust has two different types of constants which can be declared at any scope (including global)
    - `const` an unchangable value
    - `static` a possibly `mut` variable with `'static` lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is `unsafe`
*/

// globals are declared outside all other scopes
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("This is {LANGUAGE}");
    println!("The threshold is {THRESHOLD}");
    println!("{n} is {}", if is_big(n) { "big" } else { "small" });

    // error: cannot assign to a constant
    // THRESHOLD = 5;
}
