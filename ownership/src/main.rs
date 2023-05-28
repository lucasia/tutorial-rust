mod arguments;
mod return_vals;
mod tuple;
mod borrowing;
mod slice;

fn main() {

    slice::string_slice();

    let s = String::from("quick brown fox jumped");
    println!("{}", slice::first_word(&s));
    println!("{}", slice::first_word_v2(&s));

    /*
    borrowing::dangle_main();
    borrowing::calculate_length_main();
    borrowing::change_main();

     */

    /*
    tuple::tutple();

    arguments::fn_calling_ownership();
    return_vals::fn_return_vals();

    fn_string1();
    multi_bindings();
    clone_example();

     */
}

fn fn_string1() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);
}

fn multi_bindings() {

    let x = 5;
    let y = x; // copy (primitive)

    let s1 = String::from("hello");
    let s2 = s1; // move the pointer to s2.  s1 falls out of scope to avoid "double free" error.

    println!("{}, world!", s2);
}

fn clone_example() {

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1={}, s2={}", s1, s2);
}


