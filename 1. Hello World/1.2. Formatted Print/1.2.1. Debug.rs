// the `fmt::Debug` trait can be derived, meaning an implementation can be generated automatically

// this structure cannot be printed via either `fmt::Display` or `fmt::Debug`
#[allow(dead_code)]
struct UnPrintable(i32);

// this structure can be printed via `fmt::Debug` as it derives that trait
#[derive(Debug)]
struct DebugPrintable(i32);

// Even these nested types can be printed via `fmt::Debug`
#[derive(Debug)]
struct Structure(i32);
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // `fmt::Debug` on base types
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");
    // `fmt::Debug` on a custom type
    println!("Now {:?} will print!", Structure(3));
    // `fmt::Debug` on a nested custom type
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    // using the built-in pretty-printer
    println!("{:#?}", peter);
}
