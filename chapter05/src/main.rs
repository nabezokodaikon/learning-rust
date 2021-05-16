fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("location: ({}, {})", x, y);
}

fn main() {
    println!("{}", add(10, 20));
    let point = (3, 5);
    print_coordinates(&point);
}
