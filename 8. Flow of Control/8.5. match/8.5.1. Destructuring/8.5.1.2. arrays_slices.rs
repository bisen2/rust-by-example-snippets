fn main() {
    let array = [1, -2, 6];

    match array {
        // binds the second and the third elements
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        // a single value can be ignored with `_`
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored", third
        ),
        // you can also bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {}, and all the others were ignored", second
        ),

        // As arrays have length fixed at compile time, we cannot
        // match on an array of the wrong length
        // [-1, second] => println!(""), // error: pattern requires 2 elements but array has 3

        // storing the remaining elements in another array/slice
        [3, second, tail @ ..] =>
            println!("array[0]=3, array[1]={} and the other elements were {:?}", second, tail),

        // combining these patterns, we can bind the first and last
        // values, storing the rest in a single array
        [first, middle @ .., last] =>
            println!("array[0]={}, middle={:?}, array[2]={}", first, middle, last),
    }
}
