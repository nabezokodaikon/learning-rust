extern crate rand;

use std::net::IpAddr;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    {
        let home: IpAddr = "127.0.0.1".parse().unwrap();
        println!("{:?}", home);
    }

    {

        let secret_number = rand::thread_rng().gen_range(1, 101);

        loop {

            let mut guess = String::new();

            io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if guess < 1 || guess > 100 {
                println!("The secret number will be between 1 and 100");
                continue;
            }

            println!("You guessd: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
}
