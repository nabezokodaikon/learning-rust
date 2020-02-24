use std::str::FromStr;

trait Server {
    type Response;
    type Request: FromStr;

    fn handle(&self, req: Self::Request) -> Self::Response;
}

struct EchoServer;

impl Server for EchoServer {
    type Response = String;
    type Request = String;

    fn handle(&self, req: Self::Request) -> Self::Response {
        req
    }
}

// fn handle<S: Server>(server: S, req: &str) -> S::Response {
// let req = S::Request::from_str(&req).unwrap();
// server.handle(req)
// }

fn handle<S: Server<Request = String>>(server: S, req: &str) -> S::Response {
    server.handle(req.to_string())
}

// trait Foo<T> {}
// trait Bar {
// type T;
// }

trait Foo<T> {
    fn new(t: T) -> Self;
}
trait Bar {
    type T;
    fn new(t: Self::T) -> Self;
}

// fn some_fun_foo<S, T: Foo<S>>(t: T) {}
// fn some_fun_bar<T: Bar>(T: T) {}

fn some_fun_foo<T: Foo<u32>>(t: T) {}
fn some_fun_bar<T: Bar<T = u32>>(t: T) {}

struct Baz;
// impl<T> Foo<T> for Baz {}
// struct Qux<T> {};
// impl<T> Bar for Qux<T> {
// type T = T;
// }

impl Foo<i32> for Baz {}
impl Foo<char> for Baz {}

fn main() {}
