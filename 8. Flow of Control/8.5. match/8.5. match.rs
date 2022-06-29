fn main() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        // match a single value
        1 => println!("One!"),
        // match several values
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime!"),
        // match an inclusive range
        13..=19 => println!("A teen"),
        // handle the rest of cases
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
