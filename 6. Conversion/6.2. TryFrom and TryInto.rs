// Similar to From and Into, TryFrom and TryInto are generic traits
// for converting between types. These are used for failiable
// conversions and return a Result type

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err("Not an even value")
        }
    }
}

fn main() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err("Not an even value"));

    // TryInto
    let result: Result<EvenNumber, &str> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, &str> = 5i32.try_into();
    assert_eq!(result, Err("Not an even value"));
}
