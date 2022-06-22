// Implementing `fmt::Display` for sequential data is more tricky because each call to `write!` will generate a new `fmt::Result`
// We can use the `?` operator to help - this operator will check if `write!` returned an error
// - If it does, the error is returned
// - Otherwise, the function continues

use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // create a reference to the contained `Vec<i32>`
        let vec = &self.0;

        write!(f, "[")?;

        // iterate over the elements in `vec`
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
