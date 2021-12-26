use std::fmt;

use crate::vector::Vector;

#[derive(Default, Debug, Clone, Copy)]
pub struct Point<const N: usize>(Vector<N>);

impl<const N: usize> Point<N> {
    pub fn to_vector(self) -> Vector<N> {
        self.0
    }
}
impl<T: Into<Vector<N>>, const N: usize> From<T> for Point<N> {
    fn from(arg: T) -> Self {
        Self(arg.into())
    }
}
impl<const N: usize> fmt::Display for Point<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;

        for comp in self.0.components() {
            write!(f, "{},", comp)?;
        }

        write!(f, ")")
    }
}
