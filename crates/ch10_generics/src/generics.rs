use std::path::Path;
use log::debug;

pub fn generics() {

    debug!("====== generics ======");

    let number_list = vec![34,50,25,100,65];
    let result = largest(&number_list);
    assert_eq!(result, &100);

    let number_list = vec![102, 34, 6000, 53, 2, 43, 8];
    let result = largest(&number_list);
    assert_eq!(result, &6000);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    assert_eq!(result, &'y');

    debug!("====== struct generics ======");

    let point = Point {x:2, y:4.0};
    debug!("x = {}, y = {}", point.x(), point.y());

}

pub fn largest<T: std::cmp::PartialOrd + std::fmt::Display>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
           largest = item;
        }
    }
    debug!("largest is {}", largest);
    largest
}


struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}