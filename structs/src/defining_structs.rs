struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

pub fn main_defining_structs() {
    println!("Hello, world!");

    let mut user1 = User {
        active: true,
        username: String::from("foo"),
        email: String::from("foo@example.com"),
        sign_in_count: 1,
    };

    println!("email:{}", user1.email);

    user1.email = String::from("bar@example.com");
    println!("email now: {}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {}", black.0);
    println!("origin: {}", origin.0);

    let subject = AlwaysEqual;

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn build_user(email: String, username: String) -> User {
    User {
        active:true,
        username,
        email,
        sign_in_count:1,
    }
}

