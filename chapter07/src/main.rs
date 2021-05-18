#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

use std::fmt;

struct Password {
    value: String,
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.chars().map(|_| '*').collect::<String>())
    }
}

fn main() {
    {
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
    }
    {
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        user1.email = String::from("anoteremail@example.com");
    }
    {
        let user1 = build_user(String::from("ddd@example.com"), String::from("taro"));
        let user2 = User {
            email: String::from("ccc@example.com"),
            username: String::from("jiro"),
            ..user1
        };
        dbg!(user2);
    }
    {
        let black = Color(0, 0, 0);
        let Point(x, y, z) = Point(0, 0, 0);
        println!("{} {} {}", black.0, black.1, black.2);
        println!("{} {} {}", x, y, z);
    }
    {
        let a = String::from("123456789");
        println!("{}", a);

        let a = Password {
            value: String::from("123456789"),
        };
        println!("{}", a);
    }
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );

        println!("{}", rect1.can_hold(&rect1));
    }
    {
        let a = Point2::new(3., 5.);
        println!("x = {}, y = {}", a.x, a.y);
    }
    {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
    }
}

enum IpAddrKind {
    V4,
    V6,
}

struct Point2 {
    x: f64,
    y: f64,
}

impl Point2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
