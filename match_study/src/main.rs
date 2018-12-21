#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {

    {
        let v = value_in_cents(Coin::Penny);    
        println!("coin value is {}", v);
    }

    {
        value_in_cents(Coin::Quarter(UsState::Alaska));
    }

    {
        let five = Some(5);
        let six = plus_one(five);
        println!("six: {:?}", six);

        let none = plus_one(None);
        println!("none: {:?}", none);
    }

    {
        let some_u8_value = 5u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => println!("other"),
        }
    }
}