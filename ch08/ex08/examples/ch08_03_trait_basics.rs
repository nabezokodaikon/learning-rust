mod sub {
    pub mod ch08_01_trait_basics;
    pub mod ch08_02_trait_basics;
}
// use ch08_01_trait_basics::CartesianCoord;
// use ch08_01_trait_basics::Coordinates;
// use ch08_01_trait_basics::PolarCoord;
use crate::sub::ch08_02_trait_basics::print_point;

struct Matrix([[f64; 2]; 2]);

// trait LinearTransform: Coordinates {
// fn transform(self, matrix: &Matrix) -> Self;

// fn rotate(self, theta: f64) -> Self
// where
// Self: Sized,
// {
// self.transform(&Matrix([
// [theta.cos(), -theta.sin()],
// [theta.sin(), theta.cos()],
// ]))
// }
// }

// impl LinearTransform for CartesianCoord {
// fn transform(mut self, matrix: &Matrix) -> Self {
// let x = self.x;
// let y = self.y;
// let m = matrix.0;

// self.x = m[0][0] * x + m[0][1] * y;
// self.y = m[1][0] * x + m[1][1] * y;
// self
// }
// }

fn main() {
    // let p = (1.0, 0.0).to_cartesian();
    // print_point(p.rotate(std::f64::consts::PI));
}