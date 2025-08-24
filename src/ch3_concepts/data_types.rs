

pub fn integer_types() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {guess}");

    let decimal = 98_222;
    println!("Decimal: {decimal}");

    let hex = 0xff;
    println!("Hexadecimal: {hex}");

    let octal = 0o77;
    println!("Octal: {octal}");

    let binary = 0b1111_0000;
    println!("Binary: {binary}");

    let byte = b'A';
    println!("Byte: {byte}");
}

pub fn float_types() {
    let x = 2.5; // f64
    println!("The value of x is: {x}");

    let y : f32 = 3.3;
    println!("The value of y is: {y}");
}

pub fn char_types() {
    let c = 'z';
    println!("The value of c is: {c}");

    let z: char = 'ℤ'; // with explicit type annotation
    println!("The value of z is: {z}");

    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");
}

pub fn math_operations() {
    println!("addition {}", 5 + 10);
    println!("subtraction {}", 99.5 - 4.3);
    println!("multiplication {}", 4 * 30);
    println!("division {}", 56.7 / 32.2);
    println!("integer division {}", -5 / 3);
    println!("remainder / modulus {}", 43 % 5);
}

pub fn tuples() {

    let tup = (500, 6.4, 1);
    println!("The value of x is {}", tup.0);

    let (_x, y, z) = tup;
    println!("The value of y is: {y}, the value of z is {z}");
}

pub fn arrays() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    println!("first element is {}", a[0]);

    let b = [3; 5];
    println!("{:?}", b);

}
