use std::io::Read;
use std::thread;
use std::time::Duration;

fn main() {
    call_foo();
}

fn single_thread() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #1 count {}", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    println!("please wait.");
    handle.join().expect("cannot join thread.");
    println!("program end.");
}

fn multi_thread() {
    let handle1 = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #1 count {}", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    let handle2 = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #2 count {}", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    let handle3 = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #3 count {}", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    println!("please wait.");
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    println!("program end.");
}

fn closure() {
    let task = || {
        for i in 0..10 {
            println!("thread #1 count {}", i);
            thread::sleep(Duration::from_millis(1000));
        }
    };
    let handle = thread::spawn(task);
    println!("please wait.");
    handle.join().unwrap();
    println!("program end.");
}

fn foo(id: i32) {
    for i in 0..10 {
        println!("thread #{} count {}", id, i);
        thread::sleep(Duration::from_millis(1000));
    }
}

fn call_foo() {
    println!("please wait.");
    thread::spawn(|| foo(10)).join().unwrap();
    thread::spawn(|| foo(20)).join().unwrap();
    thread::spawn(|| foo(30)).join().unwrap();
    println!("program end.");
}
