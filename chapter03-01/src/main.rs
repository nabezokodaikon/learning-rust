fn add(x: i32, y: i32) -> i32 {
    println!("call add");
    x + y
}

fn main() {
    let name = "masuda tomoaki";
    let age = 30;
    let r = add(1, 2);

    let x = 100.234;
    println!("x is {}", x);

    let f = true;
    println!("f is {}", f);

    let c = 'A';
    let c = 'あ';

    let s = "Hello Rust world";
    println!("{}", s);

    let dog = "DOG";
    let cat = "CAT";
    println!("{} and {}", dog, cat);

    let s = String::from("Hello Rust world");
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = String::from("world");
    let s = s1 + " " + &s2 + " " + &s3;
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = String::from("world");
    let s = format!("{} {} {}", s1, s2, s3);
    println!("{}", s);

    let s1 = "Hello";
    let s2 = "Rust";
    let s3 = "world";
    let s = format!("{} {} {}", s1, s2, s3);
    println!("{}", s);
}
