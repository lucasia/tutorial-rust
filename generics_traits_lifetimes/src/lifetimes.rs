/*
    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime
 */
pub fn lifetimes_main() {
    inner_block();
    single_block();

    test_longest1();
    test_longest2();

    test_struct_with_reference();
}

fn inner_block() {
    let r;

    {
        let x = 5;
        r = &x;

        println!("r inner: {}", &r);
    }

    // println!("r outer: {}", &r); // compile error, x lifetime ended in inner block
}


fn single_block() {
    let r;

    let x = 5;
    r = &x;

    println!("r: {}", &r);
}

fn test_longest1() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn test_longest2() {
    let string1 = String::from("long string is long");
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2(x: &str, y: &str) -> String {
    let result = String::from("really long string");
    result
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn test_struct_with_reference() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    dbg!(i);
}