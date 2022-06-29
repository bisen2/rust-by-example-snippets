// For pointers, there is a distinction between destructuring and dereferencing
// Dereferencing uses `*`
// Destructuring uses `&`, `ref`, and `ref mut`

fn main() {
    // assign a reference of type `i32`
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // to avoid the `&` you can dereference before matching
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // what if you don't start with a reference?
    let _not_a_reference = 3;

    // Rust provides `ref` for this purpose. It modifies the
    // assignment so that a reference is created for the element
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    // use `ref` to create a reference
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // use `ref mut` similarly
    match mut_value {
        ref mut m => {
            // got a reference, gotta dereference it before we can
            // add anything to it
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
