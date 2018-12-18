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
        email: String::from("somenone@example.com"),
        username: String::from("sumeusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("somenone@example.com"),
        username: String::from("sumeusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");
    println!("user1: {}", user2.email);

    let user3 = build_user(String::from("taro@example.com"), "taro".to_string());
    println!("username: {}, email: {}", user3.username, user3.email);

    let user4 = build_user2(String::from("taro@example.com"), "taro".to_string());
    println!("username: {}, email: {}", user4.username, user4.email);

    let user5 = User {
        email: String::from("anotheremail@example"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!("username: {}, email: {}", user5.username, user5.email);

    let user6 = User {
        email: String::from("anotheremail@example"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("username: {}, email: {}", user6.username, user6.email);

    let black = Color(0, 0, 0);
    let oritin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
