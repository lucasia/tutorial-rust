mod rectangle;
mod guess;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

fn greeting(name: &str) -> String {
    format!("Hello! {name}")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add_two() {
        assert_eq!(2, add_two(0));
        assert_eq!(5, add_two(3));
        assert_eq!(1, add_two(-1));
    }

    #[test]
    fn test_greeting() {
        let name = String::from("Carol");
        let result = greeting(&name.as_str());
        assert!(result.contains(&name),
                "Greeting did not containt name '{}', value was '{}'",
                name, result
        );
    }
}
