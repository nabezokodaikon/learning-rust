fn main() {
    map();
    filter();
    find();
    max_and_min();
    zip();
}

fn map() {
    println!("use map");
    let a = [1, 2, 3, 4, 5];
    let b = a.iter().map(|x| x * x);
    b.for_each(|x| println!("it is {}", x));
}

fn filter() {
    println!("use filter");
    let a = [1, 2, 3, 4, 5];
    let b = a.iter().filter(|&x| x % 2 == 1);
    b.for_each(|x| println!("it is {}", x));
}

fn find() {
    println!("use find");
    let a = [1, 2, 3, 4, 5];
    let b = a.iter().find(|&&x| x == 3);
    let c = a.iter().find(|&&x| x > 10);
    println!("b is {:?}", b);
    println!("c is {:?}", c);
}

fn max_and_min() {
    println!("use max and min");
    let a: [i32; 0] = [];
    let max = a.iter().max();
    let min = a.iter().min();
    println!("max is {:?}", max);
    println!("min is {:?}", min);
}

fn zip() {
    println!("use zip");
    let a = [1, 2, 3, 4, 5];
    let b = ["one", "two", "three", "four", "five"];
    let c: Vec<_> = a.iter().zip(b.iter()).collect();
    c.iter().for_each(|i| println!("c is {} and {}", i.0, i.1));
}
