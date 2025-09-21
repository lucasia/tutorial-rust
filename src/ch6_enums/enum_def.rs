use log::debug;

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrKind {
    fn to_string(&self) -> String {
        match &self {
            IpAddrKind::V4(a, b, c, d) => format!("{a}.{b}.{c}.{d}"),
            IpAddrKind::V6(addr) => addr.clone(),
        }
    }
}

pub fn enum_def() {
    // enums
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    debug!("home: {:?}", home.to_string());
    debug!("loopback: {:?}", loopback.to_string());

    // option
    assert_eq!(4, safe_square(Some(2)));
    assert_eq!(0, safe_square(None));
}

fn safe_square(value: Option<i32>) -> i32 {
    match value {
        Some(x) => x * x,
        None => 0,
    }
}
