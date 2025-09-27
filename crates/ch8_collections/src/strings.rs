use log::debug;

pub fn strings() {
    hello();
    init_examples();
    updates();
    concat();
    format_example();
    iterator();
}

fn iterator() {
    let s = String::from("Зд");
    for c in s.chars() {
        debug!("c: {c}");
    }

    for b in s.bytes() {
        debug!("b: {b}");
    }
}

fn format_example() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    debug!("Game of {s}");
}

fn concat() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    assert_eq!("Hello, world!", &s3[..]);
}

fn updates() {
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);

    assert_eq!("foobar", &s[..]);

    debug!("s2 is {s2}");

    s.push('!');
    assert_eq!("foobar!", &s[..]);
}

fn hello() {
    debug!("{}", String::from("Hello"));
    debug!("{}", String::from("こんにちは"));
    debug!("{}", String::from("你好"));
    debug!("{}", String::from("Здравствуйте"));
}

fn init_examples() {
    debug!("{}", String::new());

    let data = "initial contents";

    debug!("{}", data.to_string());
    debug!("{}", "initial contents".to_string());
    debug!("{}", String::from("initial contents"));
}
