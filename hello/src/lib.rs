// src/lib.rs

// Struct definition
pub struct Guess {
    value: i32,
}

// Implementation with intentional bug (swapped panic messages)
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            // This is WRONG on purpose (for test failure demonstration)
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        } else if value > 100 {
            // This is WRONG on purpose (for test failure demonstration)
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(100);
    }
}

