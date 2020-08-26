#[derive(Debug)]
struct Person {
    name: String,
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

fn new_person(name: &str, age: i32) -> Person {
    let p = Person {
        name: name.to_string(),
        age,
    };
    p
}

fn main() {
    let mut a = Person {
        name: String::from("masuda"),
        age: 50,
    };
    println!("a is {:?}", a);

    let mut x = &mut a;
    println!("x is {:?}", x);
    x.name = String::from("kato");
    x.age = 0;
    println!("x is {:?}", x);
    println!("a is {:?}", a);
}
