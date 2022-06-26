// variable bindings have a scope and are constrained to live in a block.
// A block is a collection of statements enclosed by `{}`.

fn main() {
    // this binding lives in the main function
    let long_lived_binding = 1;

    { // this is a block and has a smaller sopce than the main function
        // this binding only exists in this block
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    } // end of the block

    // println!("outer short: {}", short_lived_binding); // error: cannot find value `short_lived_binding` in this scope

    println!("outer long: {}", long_lived_binding);

    // variable shadowing is also allowed
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);
        let shadowed_binding = "abc";
        println!("shadowed in inner block: {}", shadowed_binding);
    }

    println!("outide inner block: {}", shadowed_binding);

    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}
