struct Foo {
    x: (u32, u32),
    y: u32,
}

fn main() {
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),
        // you can destructure structs and rename variables
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i is {:?}", i),
        // and you can also ignore some variables
        Foo { y, .. } => println!("y={}, we don't care about x", y),
        // however, ignoring variables must be explicit
        // Foo { y } => println!("y={y}"), // error: missing structure fields
    }
}
