use log::debug;
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn derefs() {
    debug!("====== in deref examples ======");

    deref_int();
    deref_box();
    deref_mybox();
    deref_coersion();
}

fn deref_coersion() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // deref coersion of `hello(&(*m)[..]);`
}

fn deref_mybox() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn hello(name: &str) {
    debug!("Hello, {name}!");
}

fn deref_int() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn deref_box() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
