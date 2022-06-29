// match guards can be used to filter an arm
#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Farenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(35);

    match temperature {
        Temperature::Celsius(t) if t > 30 =>
            println!("{t}C is above 30C"),
        Temperature::Celsius(t) =>
            println!("{t}C is below 30C"),
        Temperature::Farenheit(t) if t > 86 =>
            println!("{t}F is above 86F"),
        Temperature::Farenheit(t) =>
            println!("{t}F is below 86F"),
    }

    // note: the compiler won't take guard conditions into account
    // when checking if all patterns are covered
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        // this arm is unnecessary from a logical standpoint
        // but the compiler can't figure that out
        _ => unreachable!("should never happen"),
        // the `unreachable!` macro is used to mark a point
        // that execution should never reach. If it does, the
        // program immediately panics
    }
}
