#[cfg(test)]
mod tests {
    use log::debug;
    use test_log::test;

    #[test]
    fn test_shadow() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => debug!("Got 50"),
            Some(y) => debug!("Matched, y = {y}"),
            _ => debug!("Default case, x = {x:?}"),
        }

        assert_eq!(Some(5), x);
        assert_eq!(10, y);
    }

    #[test]
    fn test_range() {
        // integer
        let x = 5;

        match x {
            1..=5 => debug!("one through five"),
            _ => debug!("something else"),
        }

        // char
        let x = 'c';

        match x {
            'a'..='j' => debug!("early ASCII letter"),
            'k'..='z' => debug!("late ASCII letter"),
            _ => debug!("something else"),
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_point() {
        let p = Point { x: 0, y: 7 };

        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);

        match p {
            Point { x, y: 0 } => debug!("On the x axis at {x}"),
            Point { x: 0, y } => debug!("On the y axis at {y}"),
            Point { x, y } => {
                debug!("On neither axis: ({x}, {y})");
            }
        }

        match p {
            Point { x, .. } => debug!("x is {x}"),
        }
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    #[test]
    fn test_enum() {
        process_turn(Message::Quit);
        process_turn(Message::Move { x: 10, y: 20 });
        process_turn(Message::ChangeColor(0, 160, 255));
        process_turn(Message::Write(String::from("hello")));
    }

    fn process_turn(msg: Message) {
        match msg {
            Message::Quit => {
                debug!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                debug!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                debug!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                debug!("Change color to red {r}, green {g}, and blue {b}");
            }
        }
    }

    #[test]
    fn test_remaining_parts() {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => debug!("First: {}, Last: {}", first, last),
        }
    }

    #[test]
    fn test_match_guard() {
        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => debug!("{x} is even"),
            Some(x) => debug!("{x} is odd"),
            None => (),
        }
    }

    enum Greeting {
        Hello { id: i32 },
    }

    #[test]
    fn test_at_bindings() {
        let msg = Greeting::Hello { id: 5 };

        match msg {
            Greeting::Hello { id: id @ 3..=7 } => {
                debug!("Found an id in range: {id}")
            }
            Greeting::Hello { id: 10..=12 } => {
                debug!("Found an id in another range")
            }
            Greeting::Hello { id } => debug!("Found some other id: {id}"),
        }
    }
}
