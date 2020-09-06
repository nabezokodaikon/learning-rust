use std::sync::mpsc;
use std::thread;
// use std::thread::JoinHandle;

// fn typename<T>(_: &T) -> &'static str {
// std::any::type_name::<T>()
// }

fn main() {
    let mut handles = Vec::new();
    // println!("{}", typename(&handles));
    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    (0..10).for_each(|_| {
        // mainから各スレッドへのチャンネル
        let (snd_tx, snd_rx) = mpsc::channel();
        // 各スレッドからmainへのチャンネル
        let (rcv_tx, rcv_rx) = mpsc::channel();

        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_rx);

        handles.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    });

    (0..10).for_each(|x| {
        let _ = snd_channels[x].send(data[x]);
    });

    (0..10).for_each(|x| {
        data[x] = rcv_channels[x].recv().unwrap();
    });

    let _ = handles.into_iter().map(|h| h.join());

    dbg!(data);
}
