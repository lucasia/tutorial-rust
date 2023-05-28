use std::io;

pub fn boolean_play() {
    let true_bool = true;

    let false_bool: bool = false; // with explicit type annotation

    let default_bool: Option<bool> = None;

    println!("{} {} {}", true_bool, false_bool, default_bool.is_none());
}

pub fn char_play() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{} {} {}", c, z, heart_eyed_cat);
}

pub fn tup_play() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{} {} {}", tup.0, tup.1, tup.2);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");
}

pub fn array_play() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("array a:  {} {} {} {} {}", a[0], a[1], a[2],  a[3], a[4]);

    let b  = [3; 5];

    println!("array b:  {} {} {} {} {}", b[0], b[1], b[2],  b[3], b[4]);
}

pub fn array_panic() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}