fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings.
    // These warnings can be silenced by prefixing the name with `_`
    let _unused_variabl = 3u32;
    // let noisy_unused_variable = 2u32; // warning: unused variable
}
