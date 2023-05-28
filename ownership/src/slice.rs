// # create a function that takes a string of words seperated by spaces.  return the first word in the string.
pub fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// # create a function that takes a string of words seperated by spaces.  return the first word in the string.
pub fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

pub fn string_slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello={}; world={}", hello, world);
}