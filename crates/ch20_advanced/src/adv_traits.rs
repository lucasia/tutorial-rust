#[cfg(test)]
mod tests {
    use log::debug;
    use std::fmt;
    use std::fmt::Write;
    use std::fmt::{Display, Formatter};
    use std::ops::Add;
    use test_log::test;

    // ===============================
    // default generics
    // ===============================
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, rhs: Point) -> Point {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    #[test]
    fn test_add_point() {
        let point1 = Point { x: 1, y: 0 };
        let point2 = Point { x: 2, y: 3 };

        assert_eq!(Point { x: 3, y: 3 }, (point1 + point2))
    }

    #[derive(Debug, PartialEq)]
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, rhs: Meters) -> Millimeters {
            Millimeters(self.0 + (rhs.0 * 1000))
        }
    }

    #[test]
    fn test_add_meters_for_mm() {
        let meters = Meters(1);
        let millimeters = Millimeters(10);

        assert_eq!(Millimeters(1010), millimeters + meters)
    }

    // ===============================
    // supertraits
    // ===============================

    trait OutlinePrint: Display {
        fn outline_print(&self) -> String {
            let output = self.to_string();
            let len = output.len();
            let mut buf = String::new();
            writeln!(buf, "{}", "*".repeat(len + 4)).unwrap();
            writeln!(buf, "*{}*", " ".repeat(len + 2)).unwrap();
            writeln!(buf, "* {output} *").unwrap();
            writeln!(buf, "*{}*", " ".repeat(len + 2)).unwrap();
            write!(buf, "{}", "*".repeat(len + 4)).unwrap();
            buf
        }
    }

    impl Display for Point {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    #[test]
    fn test_outline_print_point() {
        let p = Point { x: 2, y: 5 };

        let result = p.outline_print();
        debug!("\n{result}");
        assert!(result.contains("(2, 5)"));
    }

    // ===============================
    // newtype pattern
    // ===============================

    struct BracketWrapper(Vec<String>);

    impl Display for BracketWrapper {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    #[test]
    fn test_newtype() {
        let v = vec!["hello".to_string(), "world".to_string()];

        assert_eq!("[hello, world]", BracketWrapper(v).to_string());
    }
}
