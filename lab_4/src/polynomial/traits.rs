use std::{
    cmp,
    fmt::{Debug, Display, Formatter, Result},
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
    rc::Rc,
};

use super::Poly;
use crate::finite_field::{F2FiniteExtension, Field, FiniteField};

impl<F> PartialEq for Poly<F>
where
    F: Field,
{
    fn eq(&self, other: &Self) -> bool {
        if self.field != other.field || self.degree() != other.degree() {
            return false;
        }
        for i in 0..self.degree() + 1 {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

impl<F> Clone for Poly<F>
where
    F: Field,
{
    fn clone(&self) -> Self {
        Poly {
            field: Rc::clone(&self.field),
            data: self.data.clone(),
        }
    }
}

impl<F> Add for Poly<F>
where
    F: Field,
{
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += &other;
        self
    }
}

impl<F> Add<&Poly<F>> for Poly<F>
where
    F: Field,
{
    type Output = Self;

    fn add(mut self, other: &Self) -> Self::Output {
        self += other;
        self
    }
}

impl<F> Add<Poly<F>> for &Poly<F>
where
    F: Field,
{
    type Output = Poly<F>;

    fn add(self, mut other: Poly<F>) -> Self::Output {
        other += self;
        other
    }
}

impl<F> Add for &Poly<F>
where
    F: Field,
{
    type Output = Poly<F>;

    fn add(self, other: Self) -> Self::Output {
        let mut sum = self.clone();
        sum += other;
        sum
    }
}

impl<F> AddAssign<Poly<F>> for Poly<F>
where
    F: Field,
{
    fn add_assign(&mut self, other: Self) {
        *self += &other;
    }
}

impl<F> AddAssign<&Poly<F>> for Poly<F>
where
    F: Field,
{
    fn add_assign(&mut self, other: &Self) {
        self.data.resize(
            1 + cmp::max(self.degree(), other.degree()),
            self.field.zero(),
        );

        for i in 0..other.degree() + 1 {
            self[i] = self.field.add(self[i], other[i]);
        }

        self.update_len();
    }
}

impl<F> Sub for Poly<F>
where
    F: Field,
{
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self -= &other;
        self
    }
}

impl<F> Sub<&Poly<F>> for Poly<F>
where
    F: Field,
{
    type Output = Self;

    fn sub(mut self, other: &Self) -> Self::Output {
        self -= other;
        self
    }
}

impl<F> Sub<Poly<F>> for &Poly<F>
where
    F: Field,
{
    type Output = Poly<F>;

    fn sub(self, other: Poly<F>) -> Self::Output {
        self + -other
    }
}

impl<F> Sub for &Poly<F>
where
    F: Field,
{
    type Output = Poly<F>;

    fn sub(self, other: Self) -> Self::Output {
        let mut diff = self.clone();
        diff -= other;
        diff
    }
}

impl<F> SubAssign<Poly<F>> for Poly<F>
where
    F: Field,
{
    fn sub_assign(&mut self, other: Self) {
        *self -= &other;
    }
}

impl<F> SubAssign<&Poly<F>> for Poly<F>
where
    F: Field,
{
    fn sub_assign(&mut self, other: &Self) {
        self.data.resize(
            1 + cmp::max(self.degree(), other.degree()),
            self.field.zero(),
        );

        for i in 0..other.degree() + 1 {
            self[i] = self.field.sub(self[i], other[i]);
        }

        self.update_len();
    }
}

impl<F> Mul for Poly<F>
where
    F: Field,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        &self * &other
    }
}

impl<F> Mul<&Poly<F>> for Poly<F>
where
    F: Field,
{
    type Output = Self;

    fn mul(self, other: &Self) -> Self::Output {
        &self * other
    }
}

impl<F> Mul<Poly<F>> for &Poly<F>
where
    F: Field,
{
    type Output = Poly<F>;

    fn mul(self, other: Poly<F>) -> Self::Output {
        self * &other
    }
}

impl<F> Mul for &Poly<F>
where
    F: Field,
{
    type Output = Poly<F>;

    fn mul(self, other: Self) -> Self::Output {
        let f = self.field();
        let mut prod = Poly::zero(Rc::clone(&f), self.degree() + other.degree() + 1);

        for i in 0..self.degree() + 1 {
            for j in 0..other.degree() + 1 {
                prod[i + j] = f.add(prod[i + j], f.mul(self[i], other[j]));
            }
        }

        prod.update_len();
        prod
    }
}

impl<F> MulAssign<Poly<F>> for Poly<F>
where
    F: Field,
{
    fn mul_assign(&mut self, other: Self) {
        *self *= &other;
    }
}

impl<F> MulAssign<&Poly<F>> for Poly<F>
where
    F: Field,
{
    fn mul_assign(&mut self, other: &Self) {
        let tmp = self.clone();
        let f = tmp.field();
        self.data.iter_mut().map(|x| *x = f.zero()).count();
        self.data
            .resize(tmp.degree() + other.degree() + 1, f.zero());

        for i in 0..tmp.degree() + 1 {
            for j in 0..other.degree() + 1 {
                self[i + j] = f.add(self[i + j], f.mul(tmp[i], other[j]));
            }
        }

        self.update_len();
    }
}

impl<F> Neg for Poly<F>
where
    F: Field,
{
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        if self.field.characteristic() == 2 {
            return self;
        }
        for i in 0..self.degree() + 1 {
            self[i] = self.field.neg(self[i]);
        }
        self
    }
}

impl<F> Neg for &Poly<F>
where
    F: Field,
{
    type Output = Poly<F>;

    fn neg(self) -> Self::Output {
        let opp = self.clone();
        -opp
    }
}

impl<F> Index<usize> for Poly<F>
where
    F: Field,
{
    type Output = F::FieldElement;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<F> IndexMut<usize> for Poly<F>
where
    F: Field,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<F> Debug for Poly<F>
where
    F: F2FiniteExtension,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        let k = self.field();
        let mut i = 0;
        while i <= self.degree() {
            if self[i] != k.zero() {
                if self[i] != k.one() || i == 0 {
                    write!(f, "{:X}", k.elt_to_u32(self[i]))?;
                }
                match i {
                    0 => (),
                    1 => write!(f, "x")?,
                    _ => write!(f, "x^{}", i)?,
                }
                if i < self.degree() {
                    write!(f, " + ")?;
                }
            }
            i += 1;
        }
        Ok(())
    }
}

impl<F> Display for Poly<F>
where
    F: FiniteField,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        let k = self.field();
        let mut i = 0;
        while i <= self.degree() {
            if self[i] != k.zero() {
                if self[i] != k.one() || i == 0 {
                    write!(f, "{}", k.elt_to_str(self[i]))?;
                }
                match i {
                    0 => (),
                    1 => write!(f, "x")?,
                    _ => write!(f, "x^{}", i)?,
                }
                if i < self.degree() {
                    write!(f, " + ")?;
                }
            }
            i += 1;
        }
        Ok(())
    }
}
