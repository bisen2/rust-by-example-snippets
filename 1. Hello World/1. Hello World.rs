// Hello World program in Rust

// Two slashes denotes a comment

// Rust uses a function named `main` as the entry point
fn main() {
    // Here we call the `println!` macro - we will discuss why this is a macro (includes the `!`) rather than just a function later
    println!("Hello world!");
    println!("I'm a Rustacean!");
}

// Note the `;` at the end of the lines. This is not always required:
// - Lines ending with a `;` are treated as statements
// - Lines that do not end with a `;` are treated as expressions
// More specifically, a `;` tells the compiler to ignore the result and return `()` (the unit type)
