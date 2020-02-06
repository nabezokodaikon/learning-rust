mod ch07_01_value_scope;
use self::ch07_01_value_scope as scope;

fn main() {
    let mut p1 = scope::Parent(1, scope::Child(11), scope::Child(12));
    let p2 = p1;
    println!("p2: {:?}", p2);
    // println!("p1: {:?}", p1);
    p1 = scope::Parent(2, scope::Child(21), scope::Child(22));
    println!("p1: {:?}", p1);
}
