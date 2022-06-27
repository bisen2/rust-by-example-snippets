#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // implicit conversions are not allowed
    //let integer: u8 = decimal; // error: mismatched types

    // but explicit conversions can be done using the `as` keyword
    let integer = decimal as u8;
    let character = integer as char;

    // however, not all types have conversions defined
    // let character = decimal as char; // error: invalid cast

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16
    println!("1000 as a u16: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    println!("1000 as a u8: {}", 1000 as u8);
    // -1 + 256 = 255
    println!("-1 as a u8: {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256: {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the
    // most significant bit of that value is 1, then the value is signed

    // unless it already fits of course
    println!("128 as a i16: {}", 128 as i16);

    // 128 as u8 -> -128, whose two's compliments in eight bits is:
    println!("128 as i8 is: {}", 128 as i8);

    // 1000 as u8 -> 232
    println!("1000 as u8: {}", 1000 as u8);
    // and the two's compliment of 232 is -24
    println!("232 as a i8 is: {}", 232 as i8);

    // When casting from float to int, `as` performs a saturating cast.
    // If the floating point value exceeds the upper bound  or is less than
    // the lower bound, the returned value is the bound crossed

    // 300.0 as u8 is 255
    println!("300.0 as u8 is {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("nan as u8 is {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return unsound values. Use these methods wisely:
    unsafe {
        // 300.0 as u8 is 44
        println!("300.0 as u8 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
