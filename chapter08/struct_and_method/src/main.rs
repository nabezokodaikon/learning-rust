struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

fn print_person(pa: &Person) {
    println!("{}: {} ({}) in {}", pa.id, pa.name, pa.age, pa.addr);
}

fn add_age(pa: &mut Person) {
    pa.age += 1;
}

fn new_person(id: i32, name: &str) -> Person {
    let pa = Person {
        id: id,
        name: name.to_string(),
        age: -1,
        addr: String::from("Unknown"),
    };
    pa
}

fn main() {
    let pa = new_person(100, "masdua");
    let pa2 = new_person(200, "kato");
    let mut people = vec![pa, pa2];
    people.push(new_person(200, "yamada"));
    people.push(new_person(200, "sato"));

    for p in &people {
        print_person(p);
    }
}
