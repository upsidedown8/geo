use std::{
    fmt,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::{angle::Radians, vector::Vector};

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const M: usize, const N: usize> {
    arr: [[f32; N]; M],
}

impl Matrix<2, 2> {
    pub fn scale_xy(x: f32, y: f32) -> Self {
        Self {
            arr: [[x, 0.0], [0.0, y]],
        }
    }
    pub fn rotation<R: Into<Radians>>(angle: R) -> Self {
        let ang: Radians = angle.into();
        let ang: f32 = ang.into();

        Self {
            arr: [[ang.cos(), -ang.sin()], [ang.sin(), ang.cos()]],
        }
    }
}
impl Matrix<3, 3> {
    #[rustfmt::skip]
    pub fn scale_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            arr: [
                [x, 0.0, 0.0],
                [0.0, y, 0.0],
                [0.0, 0.0, z],
            ],
        }
    }
    pub fn rotation_about_x<R: Into<Radians>>(angle: R) -> Self {
        let ang: Radians = angle.into();
        let ang: f32 = ang.into();

        Self {
            arr: [
                [1.0, 0.0, 0.0],
                [0.0, ang.cos(), -ang.sin()],
                [0.0, ang.sin(), ang.cos()],
            ],
        }
    }
    pub fn rotation_about_y<R: Into<Radians>>(angle: R) -> Self {
        let ang: Radians = angle.into();
        let ang: f32 = ang.into();

        Self {
            arr: [
                [ang.cos(), 0.0, ang.sin()],
                [0.0, 1.0, 0.0],
                [-ang.sin(), 0.0, ang.cos()],
            ],
        }
    }
    pub fn rotation_about_z<R: Into<Radians>>(angle: R) -> Self {
        let ang: Radians = angle.into();
        let ang: f32 = ang.into();

        Self {
            arr: [
                [ang.cos(), -ang.sin(), 0.0],
                [ang.sin(), ang.cos(), 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
}
impl Matrix<4, 4> {
    pub fn scale_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            arr: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, w],
            ],
        }
    }
}
impl<const K: usize> Matrix<K, K> {
    pub fn identity() -> Self {
        let mut arr = [[0.0; K]; K];

        (0..arr.len()).for_each(|i| arr[i][i] = 1.0);

        Self { arr }
    }
    pub fn scale(factor: f32) -> Self {
        Self::identity() * factor
    }
}
impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn zero() -> Self {
        Self { arr: [[0.0; N]; M] }
    }
    pub fn get(&self, row: usize, col: usize) -> Option<f32> {
        self.arr.get(row).and_then(|r| r.get(col)).copied()
    }
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut f32> {
        self.arr.get_mut(row).and_then(|r| r.get_mut(col))
    }
}
impl<const M: usize, const N: usize> fmt::Display for Matrix<M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for r in 0..M {
            write!(f, "[")?;
            for c in 0..N {
                write!(f, "{},", self.arr[r][c])?;
            }

            if r + 1 < M {
                write!(f, "],\n ")?;
            }
        }

        writeln!(f, "]]")
    }
}

impl<const M: usize, const N: usize> From<[[f32; N]; M]> for Matrix<M, N> {
    fn from(arr: [[f32; N]; M]) -> Self {
        Self { arr }
    }
}
impl From<(f32, f32, f32, f32)> for Matrix<2, 2> {
    fn from((a, b, c, d): (f32, f32, f32, f32)) -> Self {
        Self {
            arr: [[a, b], [c, d]],
        }
    }
}

impl<const M: usize, const P: usize, const N: usize> Mul<Matrix<P, N>> for Matrix<M, P> {
    type Output = Matrix<M, N>;

    fn mul(self, rhs: Matrix<P, N>) -> Self::Output {
        let mut arr = [[0.0; N]; M];

        #[allow(clippy::needless_range_loop)]
        for row in 0..M {
            for col in 0..N {
                arr[row][col] = (0..P)
                    .map(|p| self.arr[row][p] * rhs.arr[p][col])
                    .sum::<f32>();
            }
        }

        Matrix { arr }
    }
}
impl<const M: usize, const N: usize> Mul<f32> for Matrix<M, N> {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}
impl<const M: usize, const N: usize> MulAssign<f32> for Matrix<M, N> {
    fn mul_assign(&mut self, rhs: f32) {
        self.arr
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|entry| *entry *= rhs))
    }
}
impl<const M: usize, const N: usize> Mul<Vector<N>> for &Matrix<M, N> {
    type Output = Vector<M>;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        let mut arr = [0.0; M];

        #[allow(clippy::needless_range_loop)]
        for row in 0..M {
            arr[row] = (0..N)
                .map(|p| self.arr[row][p] * rhs.get(p).unwrap())
                .sum::<f32>()
        }

        Vector::from(arr)
    }
}
impl<const M: usize, const N: usize> Add for Matrix<M, N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
impl<const M: usize, const N: usize> AddAssign for Matrix<M, N> {
    fn add_assign(&mut self, rhs: Self) {
        for r in 0..M {
            for c in 0..N {
                let entry = rhs.get(r, c).expect("matrix should be the same size");

                self.arr[r][c] += entry;
            }
        }
    }
}
impl<const M: usize, const N: usize> Sub for Matrix<M, N> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}
impl<const M: usize, const N: usize> SubAssign for Matrix<M, N> {
    fn sub_assign(&mut self, rhs: Self) {
        for r in 0..M {
            for c in 0..N {
                let entry = rhs.get(r, c).expect("matrix should be the same size");

                self.arr[r][c] -= entry;
            }
        }
    }
}
