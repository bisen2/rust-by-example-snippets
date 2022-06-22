fn main () {
    // Within `println!`, we can use `{}` to create a hole that will be interpolated with the following arguments
    println!("{} days", 21);

    // We can also specify the ordering of the arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Or we can give the arguments names
    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");

    // Special formatting can be specified after a `:`
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    println!("{number:>width$}", number=1, width=6);  // prints "     1"
    println!("{number:0>width$}", number=1, width=6); // prints "000001"

    // A `println!` call with the incorrect number of arguments will give a compiler error
    // println!("My name is {0}, {1} {0}", "Bond"); // error: invalid reference to positional argument 1
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    // Custom types cannot be automatically printed
    // println!("This struct `{}` won't print...", Structure(3)); // error: `Structure` cannot be formatted with the default formatter

    // A `println!` call can also capture external variables
    let number: f64 = 1.0;
    let width: usize = 6;
    println!("{number:>width$}"); // prints "     1"
}

// Here we have used `{}`, which accesses a type's implementation of the `fmt::Display` trait
// `{:?}` can be used to access the type's `fmt::Debug` trait, which will expose more info but in a less visually appealing manner
// Any type that implements `fmt::Display` also implements `ToString`, which allows conversion to the `String` type
