use log::debug;

pub fn integer_types() {
    let guess: u32 = "42".parse().expect("Not a number!");
    debug!("Guess: {guess}");

    let decimal = 98_222;
    debug!("Decimal: {decimal}");

    let hex = 0xff;
    debug!("Hexadecimal: {hex}");

    let octal = 0o77;
    debug!("Octal: {octal}");

    let binary = 0b1111_0000;
    debug!("Binary: {binary}");

    let byte = b'A';
    debug!("Byte: {byte}");
}

pub fn float_types() {
    let x = 2.5; // f64
    debug!("The value of x is: {x}");

    let y : f32 = 3.3;
    debug!("The value of y is: {y}");
}

pub fn char_types() {
    let c = 'z';
    debug!("The value of c is: {c}");

    let z: char = 'ℤ'; // with explicit type annotation
    debug!("The value of z is: {z}");

    let heart_eyed_cat = '😻';
    debug!("The value of heart_eyed_cat is: {heart_eyed_cat}");
}

pub fn math_operations() {
    debug!("addition {}", 5 + 10);
    debug!("subtraction {}", 99.5 - 4.3);
    debug!("multiplication {}", 4 * 30);
    debug!("division {}", 56.7 / 32.2);
    debug!("integer division {}", -5 / 3);
    debug!("remainder / modulus {}", 43 % 5);
}

pub fn tuples() {

    let tup = (500, 6.4, 1);
    debug!("The value of x is {}", tup.0);

    let (_x, y, z) = tup;
    debug!("The value of y is: {y}, the value of z is {z}");
}

pub fn arrays() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    debug!("{:?}", a);
    debug!("first element is {}", a[0]);

    let b = [3; 5];
    debug!("{:?}", b);

}
