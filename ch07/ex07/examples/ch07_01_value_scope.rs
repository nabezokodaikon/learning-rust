use std::ops::Drop;

#[derive(Debug)]
pub struct Parent(pub usize, pub Child, pub Child);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Debug)]
pub struct Child(pub usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let p1 = Parent(1, Child(11), Child(11));

    {
        let p2 = Parent(2, Child(21), Child(21));
        println!("(a) p1: {:?}, p2: {:?}", p1, p2);
    }

    println!("(b) p1: {:?}", p1);
    let p3 = Parent(3, Child(31), Child(32));
    println!("(c) p1: {:?}, p3: {:?}", p1, p3);
}
