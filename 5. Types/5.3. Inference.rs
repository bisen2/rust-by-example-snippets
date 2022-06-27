fn main() {
    // because of the annotation, the compiler knows that `elem` has type u8
    let elem = 5u8;

    // create an empty vector (growable array)
    let mut vec = Vec::new();
    // at this point the compiler deosn't know the exact type of `vec
    // it just knows that it's a vector of something (`Vec<_>`)

    // Insert `elem` in the vector
    vec.push(elem);
    // Now the compiler knows that `vec` is a `Vec<u8>`

    println!("{:?}", vec);
}
