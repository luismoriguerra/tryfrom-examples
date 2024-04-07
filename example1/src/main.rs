

#[derive(Debug)]
struct SmallNumber(u8);

impl TryFrom<i32> for SmallNumber {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 && value < 256 {
            Ok(SmallNumber(value as u8))
        } else {
            Err("Value is too big".to_string())
        }
    } 
}

impl SmallNumber {
    fn new(value: i32) -> Result<SmallNumber, String> {
        if value >= 0 && value < 256 {
            Ok(SmallNumber(value as u8))
        } else {
            Err("Value is too big".to_string())
        }
    }
}


fn main() {
    
    // let maybe_number = i32::try_from(43);

    let maybe_number = SmallNumber::try_from(43);

    match maybe_number {
        Ok(number) => println!("Number is: {:?}", number),
        Err(err) => println!("Error: {}", err),
    }

    let maybe_number = SmallNumber::try_from(300);

    match maybe_number {
        Ok(number) => println!("Number is: {:?}", number),
        Err(err) => println!("Error: {}", err),
    }

    let maybe_number = SmallNumber::new(43);

    match maybe_number {
        Ok(number) => println!("Number is: {:?}", number),
        Err(err) => println!("Error: {}", err),
    }
}
