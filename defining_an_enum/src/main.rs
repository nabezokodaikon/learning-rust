enum IpAddrKind {
    V4,
    V6,
}

struct Ipv4Addr {
}

struct Ipv6Addr {
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// enum IpAddr {
    // V4(u8, u8, u8, u8),
    // V6(String),
// }

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call!");
    }
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // route(four);
    // route(six);

    // let home = IpAddr {
        // kind: IpAddrKind::V4,
        // address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
        // kind: IpAddrKind::V6,
        // address: String::from("::1"),
    // };

    // let home = IpAddr::V4(String::from(127, 0 ,0 , 1));
    // let loopback = IpAddr::V6(String::from("::1"));
        
    let m = Message::Write(String::from("hello"));
    println!("{:?}", m);
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}

fn route(ip_type: IpAddr) {
}
