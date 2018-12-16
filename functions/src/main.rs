fn main() {
    let _x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    another_function(5, 6);

    let z = five();
    println!("The value of z is: {}", z);

    let a = plus_one(5);
    println!("The value of a is {}", a);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}, y is: {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
