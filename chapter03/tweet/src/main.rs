trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("Uoooooooooooooohhh!!!");
    }
}

struct Dove;
struct Duck;

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

fn main() {
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();

    let duck = Duck {};
    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    bird_vec.iter().for_each(|a| a.tweet());
}
