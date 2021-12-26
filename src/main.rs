use geometry::{matrix::*, point::*};
use std::f32::consts::TAU;

fn main() {
    let matrix = &Matrix::rotation(TAU / 4.0);
    let point = Point::from((0.0, 1.0)).to_vector();

    let transformed = matrix * point;

    println!("{}", transformed);
}
