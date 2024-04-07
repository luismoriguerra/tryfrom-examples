trait MyTryFromUsize {
    type Error;
    fn try_from_usize(value: usize) -> Result<Self, Self::Error> where Self: Sized;
}

impl MyTryFromUsize for u8 {
    type Error = String;

    fn try_from_usize(value: usize) -> Result<Self, Self::Error> {
        if value <= u8::MAX as usize {
            Ok(value as u8)
        } else {
            Err("Value out of range for u8".to_string())
        }
    }
}

// Standalone function that delegates to the trait implementation
fn try_u8_from_usize(value: usize) -> Result<u8, String> {
    MyTryFromUsize::try_from_usize(value)
}

fn main() {
    let small_value = try_u8_from_usize(255);
    assert_eq!(small_value, Ok(255));
    let large_value = try_u8_from_usize(1000);
    assert_eq!(large_value, Err("Value out of range for u8".to_string()));
}
