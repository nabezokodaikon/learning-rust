struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

static mut PERSON_ID: i32 = 0;

impl Person {
    fn new(name: &str, age: i32, addr: &str) -> Person {
        let id = unsafe {
            PERSON_ID += 1;
            PERSON_ID
        };
        Person {
            id,
            name: name.to_string(),
            age,
            addr: addr.to_string(),
        }
    }

    fn print(&self) {
        println!("{}: {} ({}) in {}", self.id, self.name, self.age, self.addr);
    }

    fn print_t(&self, private: bool) {
        if private {
            println!("{}: {}", self.id, self.name);
        } else {
            println!("{}: {} ({}) in {}", self.id, self.name, self.age, self.addr);
        }
    }

    fn to_str(&self) -> String {
        format!("{}: {} ({}) in {}", self.id, self.name, self.age, self.addr)
    }

    fn add_age(&mut self, n: i32) {
        self.age += 1;
    }
}

fn main() {
    let mut people = Vec::<Person>::new();
    people.push(Person::new("masuda", 50, "Tokyo"));
    people.push(Person::new("kato", 30, "Osaka"));
    people.push(Person::new("yamada", -1, "unkonwn"));
    people.push(Person::new("sato", -1, "unkonwn"));
    for p in &people {
        p.print();
    }
}
