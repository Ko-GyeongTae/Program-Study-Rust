#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let email = String::from("test@example.com");
    let username = String::from("test");
    let user = build_user(email, username);
    
    println!("{} {} {} {}", user.email, user.username, user.sign_in_count, user.active);
    println!("Hello, world!");

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    
    println!("user1 is : {:?}", user1);
    println!("user1 is : {:#?}", user1);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}