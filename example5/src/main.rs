use std::convert::{TryFrom, TryInto};

struct SmallNumber(u8);

// Implementing TryFrom for our custom type
impl TryFrom<i32> for SmallNumber {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if (0..=255).contains(&value) {
            Ok(SmallNumber(value as u8))
        } else {
            Err("Value out of range for SmallNumber")
        }
    }
}

// A function that accepts any input that can be converted into a SmallNumber using TryInto
// In this case we need to use  SmallNumber::try_from(item)
// fn print_small_number<T: TryInto<SmallNumber>>(item: T) {
//     match SmallNumber::try_from(item) {
//         Ok(small) => println!("Small number is: {}", small.0),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// Converting from i32 to SmallNumber
// with item.try_into() we force the compiler to use TryInto trait
fn print_small_number<T: TryInto<SmallNumber, Error = &'static str>>(item: T) {
    match item.try_into() {
        Ok(small) => println!("Small number is: {}", small.0),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let num: i32 = 120;

    match SmallNumber::try_from(num) {
        Ok(small) => println!("Small number is: {}", small.0),
        Err(e) => println!("Error: {}", e),
    }

    // Using try_into
    print_small_number(num);
}
