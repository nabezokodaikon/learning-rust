fn add_one_v1(x: u32) -> u32 {
    x + 1
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn put<T: std::fmt::Debug + ?Sized>(a: &T) {
    println!("{:?}", a);
}

fn main() {
    {
        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        let add_one_v3 = |x: u32| x + 1;
    }
    {
        put("hoge");
    }
}
