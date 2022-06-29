fn main() {
    // here is a naive method for looping over an enum value
    let mut optional = Some(0);
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit");
                    optional = None;
                } else {
                    println!("`i` is {i:?}`, try again");
                    optional = Some(i + 1);
                    // ^ requires three indentations
                }
            },
            _ => break,
            // ^ shouldn't be required
        }
    }

    // using `while let` makes this cleaner
    optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit");
            optional = None;
        } else {
            println!("`i` is `{i:?}`, try again");
            optional = Some(i + 1);
        }
    }
}
