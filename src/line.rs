use crate::{point::Point, vector::Vector};

pub struct Line<const N: usize> {
    origin: Point<N>,
    dir: Vector<N>,
}

impl<const N: usize> From<(Point<N>, Vector<N>)> for Line<N> {
    fn from((origin, vector): (Point<N>, Vector<N>)) -> Self {
        Self {
            origin,
            dir: vector,
        }
    }
}
