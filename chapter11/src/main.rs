enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    {
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (),
        }
    }

    {
        let a: Option<String> = Some(String::from("hello"));
        let a = match a {
            Some(x) => println!("{}", x),
            None => (),
        };
        println!("{:?}", a);
    }

    {
        let a = Account {
            name: String::from("name"),
            pass: String::from("pass"),
        };
        let Account { ref name, ref pass } = a;
        println!("{} {}", name, pass);
        println!("{} {}", a.name, a.pass);
    }

    {
        let some_u8_value = Some(0u8);
        match some_u8_value {
            Some(3) => println!("three"),
            Some(0u8) => println!("8"),
            _ => (),
        }
    }

    {
        let some_u8_value = Some(3);
        if let Some(3) = some_u8_value {
            println!("three");
        }
    }

    {
        let num = Some(4);
        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }
    }
}

struct Account {
    name: String,
    pass: String,
}
