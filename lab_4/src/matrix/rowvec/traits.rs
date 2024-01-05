use std::{
    fmt::{Debug, Display, Formatter, Result},
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::{Mat, Perm, RowVec};
use crate::finite_field::{F2FiniteExtension, Field, FiniteField};

impl<F> From<Mat<F>> for RowVec<F>
where
    F: Field,
{
    fn from(mat: Mat<F>) -> Self {
        RowVec(mat)
    }
}

impl<F> Clone for RowVec<F>
where
    F: Field,
{
    fn clone(&self) -> Self {
        RowVec(self.0.clone())
    }
}

impl<F> Add for RowVec<F>
where
    F: Field,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self + &other
    }
}

impl<F> Add<&RowVec<F>> for RowVec<F>
where
    F: Field,
{
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        RowVec(self.0 + &other.0)
    }
}

impl<F> Add<RowVec<F>> for &RowVec<F>
where
    F: Field,
{
    type Output = RowVec<F>;

    fn add(self, other: RowVec<F>) -> Self::Output {
        self + &other
    }
}

impl<F> Add for &RowVec<F>
where
    F: Field,
{
    type Output = RowVec<F>;

    fn add(self, other: Self) -> Self::Output {
        RowVec(&self.0 + &other.0)
    }
}

impl<F> AddAssign<RowVec<F>> for RowVec<F>
where
    F: Field,
{
    fn add_assign(&mut self, other: Self) {
        *self += &other;
    }
}

impl<F> AddAssign<&RowVec<F>> for RowVec<F>
where
    F: Field,
{
    fn add_assign(&mut self, other: &Self) {
        self.0 += &other.0;
    }
}

impl<F> Sub for RowVec<F>
where
    F: Field,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self - &other
    }
}

impl<F> Sub<&RowVec<F>> for RowVec<F>
where
    F: Field,
{
    type Output = Self;

    fn sub(self, other: &Self) -> Self::Output {
        RowVec(self.0 - &other.0)
    }
}

impl<F> Sub<RowVec<F>> for &RowVec<F>
where
    F: Field,
{
    type Output = RowVec<F>;

    fn sub(self, other: RowVec<F>) -> Self::Output {
        self - &other
    }
}

impl<F> Sub for &RowVec<F>
where
    F: Field,
{
    type Output = RowVec<F>;

    fn sub(self, other: Self) -> Self::Output {
        RowVec(&self.0 - &other.0)
    }
}

impl<F> SubAssign<RowVec<F>> for RowVec<F>
where
    F: Field,
{
    fn sub_assign(&mut self, other: Self) {
        *self -= &other;
    }
}

impl<F> SubAssign<&RowVec<F>> for RowVec<F>
where
    F: Field,
{
    fn sub_assign(&mut self, other: &Self) {
        self.0 -= &other.0;
    }
}

impl<F> Mul<Mat<F>> for RowVec<F>
where
    F: Field,
{
    type Output = Self;

    fn mul(self, other: Mat<F>) -> Self::Output {
        &self * &other
    }
}

impl<F> Mul<&Mat<F>> for RowVec<F>
where
    F: Field,
{
    type Output = Self;

    fn mul(self, other: &Mat<F>) -> Self::Output {
        &self * other
    }
}

impl<F> Mul<Mat<F>> for &RowVec<F>
where
    F: Field,
{
    type Output = RowVec<F>;

    fn mul(self, other: Mat<F>) -> Self::Output {
        self * &other
    }
}

impl<F> Mul<&Mat<F>> for &RowVec<F>
where
    F: Field,
{
    type Output = RowVec<F>;

    fn mul(self, other: &Mat<F>) -> Self::Output {
        RowVec(&self.0 * other)
    }
}

impl<F> MulAssign<Mat<F>> for RowVec<F>
where
    F: Field,
{
    fn mul_assign(&mut self, other: Mat<F>) {
        *self *= &other;
    }
}

impl<F> MulAssign<&Mat<F>> for RowVec<F>
where
    F: Field,
{
    fn mul_assign(&mut self, other: &Mat<F>) {
        self.0 *= other;
    }
}

impl<F> Mul<Perm> for RowVec<F>
where
    F: Field,
{
    type Output = Self;

    fn mul(self, other: Perm) -> Self::Output {
        &self * &other
    }
}

impl<F> Mul<&Perm> for RowVec<F>
where
    F: Field,
{
    type Output = Self;

    fn mul(self, other: &Perm) -> Self::Output {
        &self * other
    }
}

impl<F> Mul<Perm> for &RowVec<F>
where
    F: Field,
{
    type Output = RowVec<F>;

    fn mul(self, other: Perm) -> Self::Output {
        self * &other
    }
}

impl<F> Mul<&Perm> for &RowVec<F>
where
    F: Field,
{
    type Output = RowVec<F>;

    fn mul(self, other: &Perm) -> Self::Output {
        self.extract_cols(other.data())
    }
}

impl<F> Neg for RowVec<F>
where
    F: Field,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        RowVec(-self.0)
    }
}

impl<F> Neg for &RowVec<F>
where
    F: Field,
{
    type Output = RowVec<F>;

    fn neg(self) -> Self::Output {
        RowVec(-&self.0)
    }
}

impl<F> Index<usize> for RowVec<F>
where
    F: Field,
{
    type Output = F::FieldElement;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[(0, index)]
    }
}

impl<F> IndexMut<usize> for RowVec<F>
where
    F: Field,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[(0, index)]
    }
}

impl<F> Debug for RowVec<F>
where
    F: F2FiniteExtension,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.0)
    }
}

impl<F> Display for RowVec<F>
where
    F: FiniteField,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}
