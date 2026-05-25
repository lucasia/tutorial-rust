#[cfg(test)]
mod tests {
    use log::debug;
    use test_log::test;

    // ===============================
    // return functions
    // ===============================

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    #[test]
    fn test_function_pointers() {
        let result = do_twice(add_one, 5);
        assert_eq!(12, result);
    }

    #[test]
    fn test_numbers_to_strings() {
        let numbers = vec![1, 2, 3];

        let strings: Vec<String> = numbers.iter().map(|x| x.to_string()).collect();
        assert_eq!(vec!["1", "2", "3"], strings);

        let strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();
        assert_eq!(vec!["1", "2", "3"], strings);
    }

    #[derive(Debug, PartialEq)]
    enum Status {
        Value(u32),
        Stop,
    }

    #[test]
    fn test_list_statuses() {
        let statues = (0u32..2).map(Status::Value).collect::<Vec<_>>();
        assert_eq!(vec![Status::Value(0), Status::Value(1)], statues)
    }

    // ===============================
    // return closures
    // ===============================

    type Handler = Box<dyn Fn(i32) -> i32>;

    fn returns_closure() -> Handler {
        Box::new(|x| x + 1)
    }

    fn returns_initialized_closure(init: i32) -> Handler {
        Box::new(move |x| x + init)
    }

    #[test]
    fn test_closure() {
        let v1 = vec![1, 2, 3];

        assert_eq!(
            vec![2, 3, 4],
            v1.iter()
                .copied()
                .map(returns_closure())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_multiple_closures() {
        let handlers = vec![returns_closure(), returns_initialized_closure(123)];

        for handler in handlers {
            let output = handler(1);
            debug!("{output}");
        }
    }
}
