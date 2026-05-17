mod pattern;

#[cfg(test)]
mod tests {
    use log::info;
    use test_log::test;

    #[test]
    fn test_match_arms() {
        let optional_value = Some(1);

        match optional_value {
            None => panic!("should not be None"),
            Some(i) => assert_eq!(1, i),
        }

        let Some(value) = optional_value else {
            panic!("should not be None")
        };

        assert_eq!(1, value);
    }

    #[test]
    fn test_destructure_tuple() {
        let (x, y, z) = (1, 2, 3);

        assert_eq!(1, x);
        assert_eq!(2, y);
        assert_eq!(3, z);
    }

    #[derive(Debug, PartialEq)]
    enum Color {
        Blue,
        Green,
    }

    #[test]
    fn test_if_let() {
        let favorite_color: Option<Color> = None;
        let age: Result<u8, _> = "34".parse();

        let mut color: Option<Color> = None;

        // if let does not force us to create matches for every pattern
        if let Some(favorite_color) = favorite_color {
            color = Some(favorite_color);
        } else if let Ok(age) = age {
            color = if age > 30 {
                Some(Color::Blue)
            } else {
                Some(Color::Green)
            }
        }

        assert_eq!(Some(Color::Blue), color);
    }

    #[test]
    fn test_while_let() {
        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            for val in [1, 2, 3] {
                tx.send(val).unwrap();
            }
        });

        while let Ok(val) = rx.recv() {
            info!("Received: {val}");
        }
    }

    #[test]
    fn test_for_tuple() {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{index}: {value}");
        }
    }

    // fn print_coordinates(x: i32, y: i32) {
    //     println!("({}, {})", x, y);
    // }

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("({}, {})", x, y);
    }

    #[test]
    fn test_print_coordinates() {
        let point = (3, 5);

        print_coordinates(&point)
    }
}
