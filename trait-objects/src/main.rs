extern crate trait_objects;
use trait_objects::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw SelectBox");
    }
}

fn main() {
    {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };

        screen.run();
    }

    {
        // let screen = Screen {
        // components: vec![Box::new(String::from("Hi"))],
        // };
    }

    {
        pub trait Clone {
            fn clone(&self) -> Self;
        }
    }
}
