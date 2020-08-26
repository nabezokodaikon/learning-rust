#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32,
}

fn print_a(a: &Person) {
    println!("print_a: a is {:?}", a);
}

fn move_a(a: Person) {
    println!("move_a: a is {:?}", a);
}

fn add_age(a: &mut Person) {
    a.age += 1;
}

fn main() {
    let a = Person {
        name: "masuda",
        age: 50,
    };
    println!("a is {:?}", a);
    let x = &a;
    println!("変数xが借用する");
    println!("x is {:?}", x);
    println!("a is {:?}", a);
    let y = a;
    println!("変数yに所有権を移す");
    println!("y is {:?}", y);
    println!("a is {:?}", a);
    println!("x is {:?}", x);
}
