mod rectangle;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let black = Color(0, 1, 2);
    println!("{}", black.0);

    let origin = Point(3, 4, 5);
    println!("{}", origin.0);

    let user1 = build_user(
        String::from("someusername123"),
        String::from("someone@example.com"),
    );

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user2:");
    println!("active: {}", user2.active);
    println!("username: {}", user2.username);
    println!("email: {}", user2.email);
    println!("sign_in_count: {}", user2.sign_in_count);

    println!("--------------------------");
    rectangle::rec_fn();
}
