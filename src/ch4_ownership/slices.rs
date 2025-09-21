pub fn slices() {
    let s = String::from("hello world");
    let expected = "hello";

    assert_eq!(expected, first_word(&s));
    assert_eq!(expected, first_word_match(&s));
    assert_eq!(expected, first_word_slice(&s));

    let a = [1, 2, 3, 4, 5];
    assert_eq!(&a[1..3], &[2, 3]);
}

fn first_word(s: &String) -> &str {
    for (i, c) in s.char_indices() {
        if c.is_whitespace() {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word_match(s: &String) -> &str {
    match s.find(' ') {
        Some(index) => &s[..index],
        None => &s[..],
    }
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
