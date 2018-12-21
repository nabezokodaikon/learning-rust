pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// enum TrafficLighti2 {
    // Red2,
    // Yellow2,
    // Green2,
// }

use crate::a::series::of;
use self::a::series::of::nested_modules;
// use crate::TrafficLight::{ Red, Yellow };
use crate::TrafficLight::*;

fn main() {
    a::series::of::nested_modules();
    of::nested_modules();
    nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
