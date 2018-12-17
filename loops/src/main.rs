fn main() {
    // loop_func();
    while_func();
    while_func_2();
    for_func();
    rev_func();
}

// fn loop_func() {
    // loop {
        // println!("again!");
    // }
// }

fn while_func() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn while_func_2() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        
        index = index + 1;
    }
}

fn for_func() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn rev_func() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}
