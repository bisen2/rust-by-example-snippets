fn reverse_generic<T,U>(pair: (T, U)) -> (U, T) {
    let (fst, snd) = pair;
    (snd, fst)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // A tuple with a bunch of different types
    let long_tuple = (1u8, -1i8, 0.1f32, 0.2f64, 'a', true);

    // Values can be extracted from tuples using indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16), (4u64, -1i8));

    // Tuples are printable
    println!("Tuple of tuples: {:?}", tuple_of_tuples);

    // But too long of tuples are not printable
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple); // error: doesn't implement `fmt::Debug`

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse_generic(pair));
    let reverse = reverse_generic::<i32, bool>;
    println!("the reversed pair is {:?}", reverse(pair));

    // tuples can be deconstructed to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}
