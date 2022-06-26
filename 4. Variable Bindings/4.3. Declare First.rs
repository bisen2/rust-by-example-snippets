// It is possible to declare a variable first and initializes them later
// This is rarely used as it may lead to using unitialized variables

fn main() {
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;
    // println!("another binding: {}", another_binding); // error: borrow of possibly unitialized variable

    another_binding = 1;
    println!("another binding: {}", another_binding);
}
