// use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    handles.into_iter().for_each(|i| {
        let _ = i.join();
    });

    dbg!(data);
}
