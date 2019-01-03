use std::ops::Deref;

struct MyBox<T> {
    value: T,
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let x = 5;
        let y = MyBox { value: x };

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let m = MyBox {
            value: String::from("Rust"),
        };

        hello(&m);
    }
}
