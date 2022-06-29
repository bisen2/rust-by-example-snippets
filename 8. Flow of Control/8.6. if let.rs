enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    // in some cases, using `match` to destructure enums is awkward
    let optional = Some(7);
    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
            // ^ Needed 2 indentations just to destructure `i`
        },
        _ => {},
    };

    // in these cases, `if let` may be cleaner
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures
    // `number` into `Some(i)`, evaluate the block `{}`"
    if let Some(i) = number {
        println!("Matched: {i:?}!");
    }

    // if you need to specify a failure, use `else`
    if let Some(i) = letter {
        println!("Matched {i:?}");
    } else {
        println!("Didn't match a number.")
    }

    // provide an altered failing condition
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {i:?}");
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter");
    } else {
        println!("I don't like letters. Let's go with an emoticon");
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a { println!("a is foobar"); }
    if let Foo::Bar = b { println!("b is foobar"); }
    if let Foo::Qux(value) = c { println!("c is {value}"); }
    if let Foo::Qux(value @ 100) = c {
        println!("c = {value} = one hundred");
    }

    // if let allows us to match enum variants without parameters
    // or implementations of `PartialEq`
    enum Foo2 { Bar }

    let a = Foo2::Bar;

    // if Foo2::Bar == a { // error: `==` cannot be applied to type `Foo2`
    // ^ this fails to compile, so an `if let` is needed
    #[allow(irrefutable_let_patterns)]
    if let Foo2::Bar = a {
        println!("a is foobar");
    }
}
