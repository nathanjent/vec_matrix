use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, Sub, SubAssign,
};

pub trait Matrix<Inner> {
    /// Get the row length.
    fn row_len(&self) -> usize;
    /// Get the column length.
    fn column_len(&self) -> usize;
    /// Swaps two elements in a matrix.
    fn swap(&mut self, a: (usize, usize), b: (usize, usize));
}

#[derive(Eq, PartialEq)]
pub struct VecMatrix<T> {
    inner: Vec<T>,
    row_len: usize,
}

pub trait IntoVecMatrix {
    type Item;
    fn into_vec_matrix(self, row_len: usize) -> VecMatrix<Self::Item>;
}

impl<T: Copy> IntoVecMatrix for &[T] {
    type Item = T;
    fn into_vec_matrix(self, row_len: usize) -> VecMatrix<Self::Item> {
        VecMatrix {
            inner: self.into(),
            row_len,
        }
    }
}

impl<T> VecMatrix<T> {
    fn map<U, F: FnMut(&T) -> U>(self, op: F) -> VecMatrix<U> {
        VecMatrix {
            inner: self.inner.iter().map(op).collect(),
            row_len: self.row_len(),
        }
    }

    pub fn from_vec<U>(inner: Vec<T>, row_len: usize) -> VecMatrix<T> {
        VecMatrix { inner, row_len }
    }

    pub fn from_slice<U: Copy>(s: &[U], row_len: usize) -> VecMatrix<U> {
        VecMatrix {
            inner: s.into(),
            row_len,
        }
    }
}

impl<T> Matrix<T> for VecMatrix<T> {
    fn row_len(&self) -> usize {
        self.row_len
    }
    fn column_len(&self) -> usize {
        self.inner.len() / self.row_len()
    }
    fn swap(&mut self, (a_i, a_j): (usize, usize), (b_i, b_j): (usize, usize)) {
        let cols = self.column_len();
        self.inner.swap(a_i * cols + a_j, b_i * cols + b_j);
    }
}

impl<T> Index<(usize, usize)> for VecMatrix<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        assert!(i < self.row_len() && j < self.column_len());
        &self.inner[i * self.column_len() + j]
    }
}

impl<T> IndexMut<(usize, usize)> for VecMatrix<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        let column_len = self.column_len();
        assert!(i < self.row_len() && j < column_len);
        &mut self.inner[i * column_len + j]
    }
}

impl<T> IntoIterator for VecMatrix<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<T> Deref for VecMatrix<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for VecMatrix<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: fmt::Debug> fmt::Debug for VecMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl<T: Add<Output = T> + Copy> Add for VecMatrix<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            inner: self
                .inner
                .iter()
                .zip(rhs.inner.iter())
                .map(|(&i, &j)| i + j)
                .collect(),
            row_len: self.row_len(),
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub for VecMatrix<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            inner: self
                .inner
                .iter()
                .zip(rhs.inner.iter())
                .map(|(&i, &j)| i - j)
                .collect(),
            row_len: self.row_len(),
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul for VecMatrix<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.column_len() != rhs.row_len() {
            panic!("[A]m*n * [B]m*n iff [A]m == [B]n");
        }
        Self {
            inner: self
                .inner
                .iter()
                .zip(rhs.inner.iter())
                .map(|(&i, &j)| i * j)
                .collect(),
            row_len: self.row_len(),
        }
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for VecMatrix<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        self.map(|&i| i + rhs)
    }
}

impl<T: Sub<Output = T> + Copy> Sub<T> for VecMatrix<T> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        self.map(|&i| i - rhs)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for VecMatrix<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.map(|&i| i * rhs)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for VecMatrix<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self.map(|&i| i / rhs)
    }
}

impl<T: Rem<Output = T> + Copy> Rem<T> for VecMatrix<T> {
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        self.map(|&i| i % rhs)
    }
}

impl<T: Add<Output = T> + AddAssign<T> + Copy> AddAssign for VecMatrix<T> {
    fn add_assign(&mut self, rhs: Self) {
        for (i, &j) in self.inner.iter_mut().zip(rhs.inner.iter()) {
            *i += j;
        }
    }
}

impl<T: Sub<Output = T> + SubAssign<T> + Copy> SubAssign for VecMatrix<T> {
    fn sub_assign(&mut self, rhs: Self) {
        for (i, &j) in self.inner.iter_mut().zip(rhs.inner.iter()) {
            *i -= j;
        }
    }
}

impl<T: Mul<Output = T> + MulAssign<T> + Copy> MulAssign for VecMatrix<T> {
    fn mul_assign(&mut self, rhs: Self) {
        for (i, &j) in self.inner.iter_mut().zip(rhs.inner.iter()) {
            *i *= j;
        }
    }
}

impl<T: Add<Output = T> + AddAssign<T> + Copy> AddAssign<T> for VecMatrix<T> {
    fn add_assign(&mut self, rhs: T) {
        for i in self.inner.iter_mut() {
            *i += rhs;
        }
    }
}
