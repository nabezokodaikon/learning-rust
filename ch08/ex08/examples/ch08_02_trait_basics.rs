use crate::ch08_01_trait_basics::CartesianCoord;
use crate::ch08_01_trait_basics::Coordinates;
use crate::ch08_01_trait_basics::PolarCoord;

pub fn print_point<P: Coordinates>(point: P) {
    let p = point.to_cartesian();
    println!("({}, {})", p.x, p.y);
}

fn print_point2<P>(point: P)
where
    P: Coordinates,
{
    let p = point.to_cartesian();
    println!("({}, {})", p.x, p.y);
}

fn print_point3(point: impl Coordinates) {
    let p = point.to_cartesian();
    println!("({}, {})", p.x, p.y);
}

fn doube_point<P: Coordinates>(point: P) -> P {
    let mut cart = point.to_cartesian();
    cart.x *= 2.0;
    cart.y *= 2.0;
    P::from_cartesian(cart)
}

fn make_point<T>(x: T, y: T) -> CartesianCoord
where
    (T, T): Coordinates,
{
    (x, y).to_cartesian()
}

trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

fn to<T>(i: i32) -> T
where
    i32: ConvertTo<T>,
{
    i.convert()
}

fn main() {
    print_point((1.0, 1.0));
    print_point2((2.0, 2.0));
    print_point3((3.0, 3.0));

    print_point(PolarCoord {
        r: 1.0,
        theta: std::f64::consts::PI / 2.0,
    });

    // print_point("string");
}
