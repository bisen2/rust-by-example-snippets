fn print_num(n: i32) {
    if n % 15 == 0 {
        println!("fizzbuzz");
    } else if n % 3 == 0 {
        println!("fizz");
    } else if n % 5 == 0 {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn print_name(name: &&str) {
    match name {
        &"Ferris" => println!("There is a rustacean among us"),
        _ => println!("Hello {}", name),
    }
}

fn main() {

    // `n..m` includes `n` but not `m`
    for n in 1..101 {
        print_num(n);
    }

    // `n..=m` includes both `n` and `m`
    for n in 1..=100 {
        print_num(n);
    }

    // `for .. in` can be used to iterate through iterators
    // there are a few ways to do so

    let names = vec!["Bob", "Frank", "Ferris"];

    // `iter` borrows each element of the collection through each iteration. Thus, it leaves the collection untouched
    for name in names.iter() {
        print_name(name);
    }

    // `into_iter` consumes the collection so that on each
    // iteration the exact data is provided. Once the collection
    // is consumedit is no longer available for reuse
    for name in names.into_iter() {
        print_name(&name);
    }

    // println!("names: {:?}", names); // error: borrow of moved value

    // `iter_mut` mutably borrows each element in the collection
    // this allows the collection to be modified in place
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
