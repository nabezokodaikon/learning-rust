// const MAX_POINTS: u32 = 100_100;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    main_01();
    main_02();
    main_03();
    main_04();
    tuple();
    array();
}

fn main_01() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "  ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}

fn main_02() {
   let guess: u32 = "42".parse().expect("Not a number!"); 
   println!("The value of guess is: {}", guess);

   let x: u32 = 0xff;
   println!("The value of x is: {}", x);
}

fn main_03() {
    let x = 2.1;
    println!("The value of x is: {}", x);

    let y: f32 = 2.2;
    println!("The value of y is: {}", y);
}

fn main_04() {
    let a = 'a';
    println!("The value of a is: {}", a);

    let b = "abc";
    println!("The value of b is: {}", b);
}

fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let second = tup.1;
    println!("The value of second is: {}", second);
}

fn array() {
    let a = [1, 2, 3, 4, 5];
    println!("The value of a[2] is: {}", a[2]);
}
