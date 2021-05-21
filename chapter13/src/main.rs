use std::fmt::Display;

pub trait Geometry {
    fn area(&self) -> f64;
    fn name(&self) -> &str {
        return "Geometry";
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Geometry for Rectangle {
    fn area(&self) -> f64 {
        self.width as f64 * self.height as f64
    }
    fn name(&self) -> &str {
        return "Rectangle";
    }
}

struct Triangle {
    bottom: u32,
    height: u32,
}

impl Geometry for Triangle {
    fn area(&self) -> f64 {
        self.bottom as f64 * self.height as f64 * 0.5
    }
    fn name(&self) -> &str {
        return "Triangle";
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl From<f64> for Point {
    fn from(input: f64) -> Self {
        Point { x: input, y: input }
    }
}

fn main() {
    {
        let a = Rectangle {
            width: 10,
            height: 20,
        };
        let b = Triangle {
            bottom: 20,
            height: 5,
        };
        println!("{} area={}", a.name(), a.area());
        println!("{} area={}", b.name(), b.area());
    }
    {
        let p = Pair::new(1, 2);
        p.cmp_display();
    }
    {
        let p1 = Point::from(1.0);
        let p2: Point = (1.0).into();
        println!("{:?} {:?}", p1, p2);
    }
}
