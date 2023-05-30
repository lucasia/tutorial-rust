pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess must be greater than 1, got {}", value);
        }

        if value > 100 {
            panic!("Guess must be less than or equal to 100, got {}", value);
        }

        Guess {
            value,
        }
    }
}


#[cfg(test)]
mod test_guess {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn test_validation() {
        Guess::new(200);
    }

}