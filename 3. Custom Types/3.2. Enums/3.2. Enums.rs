// defining an enum
enum WebEvent {
    // cases can be unit-like
    PageLoad,
    PageUnload,
    // tuple-like
    KeyPress(char),
    Paste(String),
    // or struct-like
    Click { x: i64, y: i64 },
}

// handling an enum with a `match`
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted\"{}\".", s),
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}.", x, y),
    }
}

// We can use type aliases to improve readability
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
fn stringify(op: &Operations) -> &str {
    match op {
        Operations::Add => "Plus",
        Operations::Subtract => "Minus",
    }
}

// A `Self` alias is automatically generated within implementations
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // to_owned() creates an owned `String` from a string slice
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let addition = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    let add_str = stringify(&addition);
    println!("{}", add_str);

    let (x, y) = (5, 2);
    println!("{x} - {y} = {}", Operations::Subtract.run(x, y));
}
