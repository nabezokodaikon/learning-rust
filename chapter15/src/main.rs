use std::cell::Cell;
use std::{cell::RefCell, rc::Rc};

fn main() {
    {
        let a = Rc::new(10);
        dbg!(Rc::strong_count(&a));

        let b = Rc::clone(&a);
        dbg!(Rc::strong_count(&a));
        dbg!(Rc::strong_count(&b));
    }
    {
        let a = Cell::new(10);
        dbg!(a.get());
        a.set(20);
        dbg!(a.get());
    }
    {
        let a = Cell::new(10);
        let b = a.replace(20);
        dbg!(a.get());
        dbg!(b);

        let c = a.into_inner();
        dbg!(c);
        // dbg!(a);
    }
    {
        let mut a = Rc::new(10);
        // *a = 20;
    }
    {
        let a = Rc::new(Cell::new(10));
        a.set(20);
        dbg!(a.get());

        let b = Rc::clone(&a);
        b.set(30);
        dbg!(a.get());
    }
    {
        let a = Rc::new(RefCell::new(String::from("hoge")));
        dbg!(a.borrow());

        *(a.borrow_mut()) = String::from("foo");
        dbg!(a.borrow());

        let b = Rc::clone(&a);
        *(b.borrow_mut()) = String::from("bar");
        dbg!(a.borrow());

        let c = a.borrow_mut();
        // let d = a.borrow_mut();
    }
}
