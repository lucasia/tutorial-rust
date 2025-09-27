use log::{debug, info};

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

pub fn struct_def() {
    user_example();

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    debug!("black: {} {} {}", black.0, black.1, black.2);
    debug!("origin: {} {} {}", origin.0, origin.1, origin.2);
}

fn user_example() {
    let mut user1 = build_user(
        String::from("Mickey Mouse"),
        String::from("mickey.mouse@example.com"),
    );

    info!("Email before: {}", user1.email);

    user1.email = String::from("mickey.mouse@disney.com");

    info!("Email after: {}", user1.email);

    // only update email, rest are from user 1 (".." syntax)
    let user2 = User {
        email: String::from("donald.duck@example.com"),
        username: String::from("Donald Duck"),
        ..user1
    };

    assert_eq!(user1.active, user2.active);
    assert_eq!(user1.sign_in_count, user2.sign_in_count);
    assert_ne!(user1.email, user2.email);
    assert_ne!(user1.username, user2.username);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
