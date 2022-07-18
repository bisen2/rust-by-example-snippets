fn main () {
    // Variables can be type annotated
    let _logical: bool = true;

    let _a_float: f64 = 1.0; // Regular annotation
    let _an_integer = 5i32; // Suffix annotation

    // Defaults will also be used based on the literal's type
    let _default_float = 3.0; // `f64`
    let _default_integer = 7; // `i32`

    // A type can also be inferred by the context in which it is used
    let mut _inferred_type = 12; // this would be `i32`
    _inferred_type = 4294967296i64; // but is determined to be a `i64` due to the usage on this line

    // A mutable variable's value can be changed
    let mut _mutable = 12;
    _mutable = 21;
    // But a mutable variable's type cannot be changed
    // _mutable = true; // mismatched types expected integer, found `bool`

    // However, variables can be overwritten via shadowing
    let _mutable = true;
    // At this point the old `_mutable : i32` no longer exists, it was shadowed by `_mutable : bool`
}
