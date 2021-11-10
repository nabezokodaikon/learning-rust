use std::rc::Rc;

struct Resource {
    v: i32,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("drop!");
    }
}

fn main() {
    {
        let r = Resource { v: 1 };
        println!("{}", r.v);
    }
    {
        let a = Box::new(10);
        let a = Box::<i32>::new(20);
        let a = 30;
        let b = Box::new(a);
        let c = *b;
    }
    {
        let a = Rc::new(10);
        let b = Rc::clone(&a);
        println!("{} {}", a, b);
    }
}
