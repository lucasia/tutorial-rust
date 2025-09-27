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

    assert!(rect1.can_hold(&Rectangle {
        width: 10,
        height: 20
    }));

    let square1 = Rectangle::square(35);
    assert!(!rect1.can_hold(&square1));
}
