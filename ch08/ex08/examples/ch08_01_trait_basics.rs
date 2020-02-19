// 座標
pub trait Coordinates {
    fn to_cartesian(self) -> CartesianCoord;
    fn from_cartesian(cart: CartesianCoord) -> Self;
}

// デカルト座標
pub struct CartesianCoord {
    pub x: f64,
    pub y: f64,
}

impl Coordinates for CartesianCoord {
    fn to_cartesian(self) -> CartesianCoord {
        self
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        cart
    }
}

// 極座標
pub struct PolarCoord {
    pub r: f64,
    pub theta: f64,
}

impl Coordinates for PolarCoord {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        PolarCoord {
            r: (cart.x * cart.x + cart.y * cart.y).sqrt(),
            theta: (cart.y / cart.x).atan(),
        }
    }
}

impl Coordinates for (f64, f64) {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.0,
            y: self.1,
        }
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        (cart.x, cart.y)
    }
}

fn main() {
    let point = (1.0, 1.0);

    let c = point.to_cartesian();
    println!("x = {}, y = {}", c.x, c.y);

    let p = PolarCoord::from_cartesian(c);
    println!("r = {}, 0 = {}", p.r, p.theta);
}
