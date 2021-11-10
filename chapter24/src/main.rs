mod module1;
use module1::bar;
use module1::foo;

fn main() {
    bar::call_bar();
    foo::call_foo();
    println!("Hello, world!");
}
