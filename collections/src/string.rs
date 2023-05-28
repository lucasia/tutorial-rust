pub fn string_main() {

    iter_example();
    indexing_example();
    concat_examples();
    grow_examples();
    create_examples();

}

fn create_examples() {
    let mut s1 = String::new();


    let data = "initial contents";

    let s2 = data.to_string();  // first way
    let s2 = "initial contents".to_string(); // second way
    let s2 = String::from("initial contents"); // third way

    let hello = String::from("你好"); // UTF-8 encoded
}

fn grow_examples() {

    let mut s1 = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2);

    println!("s2 is {s2}");
}

fn concat_examples() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world !");

    let s3 = s1 + &s2;  // s1 moved, can no longer be used

    println!("s3 is {s3}");

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");


    let s7 = format!("{s4}-{s5}-{s6}"); // option 1
    println!("s7 is {s7}");

    let s8 = s4 + "-" + &s5 + "-" + &s6; // option 2
    println!("s8 is {s8}");
}

fn indexing_example() {

    // doesn't work!
    let s1 = String::from("jello");
    // let j = s1[0]; // <-- doesn't compile, `String` cannot be indexed by `{integer}`

    // works!
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s is {s}");
}

fn iter_example() {
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}