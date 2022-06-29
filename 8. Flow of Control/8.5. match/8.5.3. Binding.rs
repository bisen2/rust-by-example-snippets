fn age() -> u32 { 15 }

fn some_number() -> Option<u32> { Some(42) }

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // could match 1..=12 directly, but then what age would
        // the child be? Instad, bind to `n`
        n @ 1 ..= 12 => println!("I'm a child of age {n:?}"),
        n @ 13 ..= 19 => println!("I'm a teen of age {n:?}"),
        n => println!("I'm an old person of age {n:?}"),
    }

    match some_number() {
        // Got `Some` variant, match if its value (bound to `n`) is 42
        Some(n @ 42) => println!("The Answer: {n}!"),
        Some(n) => println!("Not interesting... {n}"),
        None => (),
    }
}
