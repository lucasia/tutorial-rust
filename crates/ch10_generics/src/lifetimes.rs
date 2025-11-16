use log::debug;
use std::fmt::{Display, Formatter, Result};

pub fn lifetimes() {
    valid_lifetime();

    // =========== functions ===========

    let string1 = String::from("abcd");

    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        assert_eq!(result, string1);
    }

    // =========== structs ===========
    let novel = String::from("Call me Ishmael.  Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    debug!("{}", i.part);

    longest_with_announcement(&"ab".to_string(), &"123".to_string(), i);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> Display for ImportantExcerpt<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.part)
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn longest_with_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}

fn valid_lifetime() {
    let x = 5; // ----------+-- 'b
    let r = &x; // --+-- 'a  |
    debug!("r: {r}"); //   |       |
}

// Below is an example of an invalid lifetime
// fn invalid_lifetime() {
//     let r;                      // ---------+-- 'a
//                                 //          |
//     {                           //          |
//         let x = 5;              // -+-- 'b  |
//         r = &x;                 //  |       |
//     }                           // -+       |
//                                 //          |
//     debug!("r: {r}");           //          |
//                                 // ---------+
// }
