#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        #[allow(unused_labels)]
        'inner: loop {
            println!("Entered the inner loop");

            // this would break only the inner loop
            // break;

            // this breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }
    println!("Exited the outer loop")
}
