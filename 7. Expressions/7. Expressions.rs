#![allow(path_statements)]
#![allow(unused_must_use)]

fn main() {
    // Rust programs are made up primarily of statements
    // the most common are variable binding and an
    // expression followed by a `;`

    let x = 5; // variable binding
    x; // expression
    x + 1;
    15;

    // blocks are also expressions, so the can be used as
    // values in assignments. The last expression in the block
    // will be bound
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        // this expression will be assigned to `y`:
        x_cube + x_squared + x
    };

    let z = {
        // the semicolon suppresses this expression and `()`
        // is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
