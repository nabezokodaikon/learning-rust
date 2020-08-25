fn main() {
    let rect = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    let tri = Triangle {
        base: 10.0,
        height: 20.0,
    };
    let cir = Circle { radius: 10.0 };

    println!("rect {} {}", rect.expr_str(), rect.calc_area());
    println!("rect {} {}", tri.expr_str(), tri.calc_area());
    println!("rect {} {}", cir.expr_str(), cir.calc_area());

    let s = String::from("100");
    let n = s.to_i();
    println!("n is {}", n);
}

trait ToNumber {
    fn to_i(&self) -> i32;
}

impl ToNumber for String {
    fn to_i(&self) -> i32 {
        match self.parse::<i32>() {
            Ok(n) => n,
            Err(_) => 0,
        }
    }
}

struct Rectangle {
    width: f32,
    height: f32,
}

struct Triangle {
    base: f32,
    height: f32,
}

struct Circle {
    radius: f32,
}

trait CalcArea {
    fn calc_area(&self) -> f32;
}

trait ExprString {
    fn expr_str(&self) -> String {
        "幅 x  高さ = ".to_string()
    }
}

impl ExprString for Rectangle {}
impl ExprString for Triangle {
    fn expr_str(&self) -> String {
        "底辺 x 高さ / 2 = ".to_string()
    }
}
impl ExprString for Circle {
    fn expr_str(&self) -> String {
        "円周率  x 半径 x 半径 = ".to_string()
    }
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f32 {
        self.width * self.height
    }
}

impl CalcArea for Triangle {
    fn calc_area(&self) -> f32 {
        self.base * self.height * 0.5
    }
}

impl CalcArea for Circle {
    fn calc_area(&self) -> f32 {
        self.radius * self.radius * 3.14
    }
}

fn print<T>(a: &[T])
where
    T: std::fmt::Debug,
{
    print!("a is ");
    for i in a {
        print!("{:?} ", i);
    }
    println!();
}
