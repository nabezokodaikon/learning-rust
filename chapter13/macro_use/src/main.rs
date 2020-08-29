// #[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {}: `{}` in {} }}", self.id, self.name, self.addr)
    }
}

fn main() {
    sub()
}

fn sub() {
    subsub()
}

fn subsub() {
    panic!("このプログラムは動きません。");
}
