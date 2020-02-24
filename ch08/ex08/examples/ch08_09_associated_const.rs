mod ch08_01_trait_basics;

use crate::ch08_01_trait_basics::CartesianCoord;

trait Dimension {
    const DIMENSHION: u32;
}

impl Dimension for CartesianCoord {
    const DIMENSHION: u32 = 2;
}

fn main() {
    let dim = CartesianCoord::DIMENSHION;
    const DM: u32 = CartesianCoord::DIMENSHION;
}
