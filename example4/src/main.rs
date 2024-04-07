
#[derive(Debug, PartialEq)]
struct SafeDate {
    year: i32,
    month: u8,
    day: u8,
}

impl TryFrom<(i32, u8, u8)> for SafeDate {
    type Error = String;

    fn try_from(value: (i32, u8, u8)) -> Result<Self, Self::Error> {
        let (year, month, day) = value;
        if year > 0 && (1..=12).contains(&month) && (1..=31).contains(&day) {
            Ok(SafeDate { year, month, day })
        } else {
            Err("Invalid date".to_string())
        }
    }
}

// Usage
fn main() {
    let date = SafeDate::try_from((2020, 7, 8));
    assert_eq!(date, Ok(SafeDate { year: 2020, month: 7, day: 8 }));
    let invalid_date = SafeDate::try_from((2020, 13, 32));
    assert_eq!(invalid_date, Err("Invalid date".to_string()));
}
