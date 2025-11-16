use log::debug;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(num: u64) -> u64 {
    add(num, 2)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        let guess = Guess { value };
        debug!("Guessed number: {}", guess.value);
        guess
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_add_two() {
        let result = add_two(2);
        assert_eq!(result, 4, "result should be four");
    }

    #[test]
    fn guess_new() {
        let value = 10;
        let guess = Guess::new(value);
        assert_eq!(value, guess.value);
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn guess_greater_than_100() {
        Guess::new(200);
    }
}
