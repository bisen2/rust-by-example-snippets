//! This is a library doc comment. They are denoted by the `!` after the two slashes and can be used to provide information about the containing structure
//! This information will be visible in tool tips and compiler generated info
fn main() {
    // This is a line comment - the two slashes create a comment until the end of the line

    /*
     * This is a block comment - the slash-star creates a comment until the matching star-slash
    */

    /*
    We can also do block comments without the stars on each line
    They are just stylistic
    */

    let _x = 5 + /* We can also use block comments within a line */ 5;
}
