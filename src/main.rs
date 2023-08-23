mod capabilities;
mod error;
mod prodc;

use crate::capabilities::*;
use crate::prodc::Prod;

fn main() {
    let prod_world = Prod {};
    run(prod_world)
}

fn run<W: Print>(world: W) {
    world.println("Hello World!")
}
