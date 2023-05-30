use std::cmp::PartialOrd;

pub fn generics_main() {
    test_find_largest();
    test_struct();
}

fn test_find_largest() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);


    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T,U> Point<T, U> {

    fn x(&self) -> &T {
        &self.x
    }

}

pub fn test_struct() {
    let integer = Point{x:5, y:10};
    let float = Point{x:1.0, y:4.0};

    dbg!(&integer, &float);

    let mixed = Point{x:5, y:4.5};
    dbg!(&mixed);

    println!("integer.x = {}", integer.x());
}


enum Option<T> {
    Some(T),
    None,
}
