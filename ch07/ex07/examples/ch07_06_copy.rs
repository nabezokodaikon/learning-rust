#[derive(Copy, Clone, Debug)]
struct Parent(usize, Child, Child);

#[derive(Copy, Clone, Debug)]
struct Child(usize);

fn main() {
    let p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;
    println!("{:?}", p2);
    println!("{:?}", p1);

    let b1 = true;
    let b2 = b1;
    println!("{:?}", b2);
    println!("{:?}", b1);
}
