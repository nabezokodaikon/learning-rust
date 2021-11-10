use std::ops::Add;

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn main() {
    {
        let origin = Point { x: 0, y: 0, z: 0 };
        match origin {
            Point { x, .. } => println!("x is {}", x),
        }
    }
    {
        let p = Point { x: 1, y: 2, z: 3 };
        let r = p.add(Point { x: 4, y: 5, z: 6 });
        println!("{}, {}, {}", r.x, r.y, r.z);
    }
}
