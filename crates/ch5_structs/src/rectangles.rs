use log::debug;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // constructor example for a rectangle that's a Square!
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn rectangles() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };

    // print out the rectangle
    debug!("~~~ Rectangle is  {rect1:#?}");
    dbg!(&rect1);

    // calc area
    let area = rect1.area();
    assert_eq!(area, 1200);

    let square1 = Rectangle::square(35);
    assert!(!rect1.can_hold(&square1));
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Rectangle {
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::new(8, 4);
        let smaller = Rectangle::new(4, 1);

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_itself() {
        let rect = Rectangle::new(2, 5);

        assert!(!rect.can_hold(&rect));
    }
}
