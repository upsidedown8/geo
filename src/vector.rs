use std::{
    fmt,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize> {
    arr: [f32; N],
}

impl Vector<3> {
    pub fn cross(&self, other: Self) -> Self {
        let ax = self.arr[0];
        let ay = self.arr[1];
        let az = self.arr[2];

        let bx = other.arr[0];
        let by = other.arr[1];
        let bz = other.arr[2];

        Self {
            arr: [ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx],
        }
    }
}
impl<const N: usize> Vector<N> {
    pub fn dot(&self, other: &Self) -> f32 {
        self.arr
            .iter()
            .zip(other.arr.iter())
            .map(|(&a, &b)| a * b)
            .sum::<f32>()
    }
    pub fn normalize(&mut self) {
        let abs = self.abs();

        if abs != 0.0 {
            let multiplier = 1.0 / self.abs();

            self.arr.iter_mut().for_each(|elem| *elem *= multiplier);
        }
    }
    pub fn abs(&self) -> f32 {
        self.dot(self)
    }
    pub fn components(&self) -> impl Iterator<Item = f32> + '_ {
        self.arr.iter().copied()
    }
    pub fn get(&self, idx: usize) -> Option<f32> {
        self.arr.get(idx).copied()
    }
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut f32> {
        self.arr.get_mut(idx)
    }
}
impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self { arr: [0.0; N] }
    }
}
impl<const N: usize> fmt::Display for Vector<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for comp in self.components() {
            write!(f, "{},", comp)?;
        }

        write!(f, "]")
    }
}

impl<const N: usize> From<f32> for Vector<N> {
    fn from(v: f32) -> Self {
        Self { arr: [v; N] }
    }
}
impl<const N: usize> From<[f32; N]> for Vector<N> {
    fn from(arr: [f32; N]) -> Self {
        Self { arr }
    }
}
impl From<(f32, f32)> for Vector<2> {
    fn from((x, y): (f32, f32)) -> Self {
        Self { arr: [x, y] }
    }
}
impl From<(f32, f32, f32)> for Vector<3> {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self { arr: [x, y, z] }
    }
}
impl From<(f32, f32, f32, f32)> for Vector<4> {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        Self { arr: [x, y, z, w] }
    }
}

impl<const N: usize> Add for Vector<N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
impl<const N: usize> AddAssign for Vector<N> {
    fn add_assign(&mut self, rhs: Self) {
        for (idx, comp) in rhs.components().enumerate() {
            self.arr[idx] += comp;
        }
    }
}
impl<const N: usize> Sub for Vector<N> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}
impl<const N: usize> SubAssign for Vector<N> {
    fn sub_assign(&mut self, rhs: Self) {
        for (idx, comp) in rhs.components().enumerate() {
            self.arr[idx] -= comp;
        }
    }
}
impl<const N: usize> Mul<f32> for Vector<N> {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}
impl<const N: usize> MulAssign<f32> for Vector<N> {
    fn mul_assign(&mut self, rhs: f32) {
        self.arr.iter_mut().for_each(|e| *e *= rhs)
    }
}
