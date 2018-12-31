extern crate publishing;

use publishing::kinds::PrimaryColor;
use publishing::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
