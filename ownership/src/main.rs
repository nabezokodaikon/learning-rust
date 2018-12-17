fn main() {

    println!("example_1");
    example_1();

    println!("example_2");
    example_2();

    println!("example_3");
    example_3();

    println!("example_4");
    example_4();

    println!("example_5");
    let s = String::from("hello");
    takes_ownership(s);
    // borrow of moved value: `s`
    // println!("{}", s) 

    let x = 5;
    makes_copy(x);
    println!("copy of x is {}", x);

    {
        println!("example_6");
        
        let s1 = gives_ownership();
        println!("s1: {}", s1);

        let s2 = String::from("hello");
        println!("s2: {}", s2);

        let s3 = takes_and_gives_back(s2);
        println!("s3: {}", s3);
        // borrow of moved value: `s2`
        // println!("s2: {}", s2);
    }

    {
        println!("example_7");
        let s1 = String::from("hello");
        println!("s1: {}", s1);
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
        // borrow of moved value: `s1`
        // println!("s1: {}", s1);
    }
}

fn example_1() {
    let mut s = String::from("hello");
    println!("value is {}", s);

    s.push_str(", world!");
    println!("value is {}", s);
}

fn example_2() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn example_3() {
    let s1 = String::from("hello");
    let s2 = s1;

    // use of moved value
    // println!("value is {}", s1);
    println!("value is {}", s2);
}

fn example_4() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
