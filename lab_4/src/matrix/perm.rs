use rand::Rng;
use std::ops::{Index, IndexMut, Mul};

use super::ColVec;
use crate::finite_field::Field;

#[derive(Debug, Eq, PartialEq)]
pub struct Perm(Vec<usize>);

impl Index<usize> for Perm {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Perm {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Perm {
    /// Creates a new permutation
    ///
    /// vec is the vector of images (of the indices)
    /// e.g. the permutation (0 1 2) on the set [0, 1, 2]
    /// is obtained by Perm::new([1, 2, 0]).
    ///
    /// # Panics
    ///
    /// Panics if vec contains an out of range image
    /// or the same image twice.
    pub fn new(vec: Vec<usize>) -> Self {
        let n = vec.len();
        let mut list = vec![false; n];
        for i in 0..n {
            if vec[i] >= n {
                panic!("Invalid image");
            }
            if list[vec[i]] == false {
                list[vec[i]] = true;
            } else {
                panic!("Image has already been assigned");
            }
        }
        Perm(vec)
    }

    pub fn random(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut cols = Vec::with_capacity(n);
        for i in 0..n {
            cols.push(i);
        }

        let mut vec = vec![0; n];
        for i in 0..n {
            let index = rng.gen_range(0, cols.len());
            vec[i] = cols[index];
            cols.swap_remove(index);
        }
        Perm(vec)
    }

    pub fn identity(n: usize) -> Self {
        let mut vec = Vec::with_capacity(n);
        for i in 0..n {
            vec.push(i);
        }
        Perm(vec)
    }

    pub fn copy_identity(&mut self) {
        for i in 0..self.len() {
            self[i] = i;
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn data(&self) -> &Vec<usize> {
        &self.0
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }

    pub fn inverse(&self) -> Self {
        let mut inv = vec![0; self.len()];
        for i in 0..self.len() {
            inv[self[i]] = i;
        }
        Perm(inv)
    }

    pub fn is_permutation(&self) -> bool {
        let n = self.len();
        let mut list = vec![false; n];
        for i in 0..n {
            match list.get(self[i]) {
                Some(false) => {
                    list[self[i]] = true;
                }
                _ => return false,
            }
        }
        true
    }
}

impl<F> Mul<ColVec<F>> for Perm
where
    F: Field,
{
    type Output = ColVec<F>;

    fn mul(self, other: ColVec<F>) -> Self::Output {
        &self * &other
    }
}

impl<F> Mul<&ColVec<F>> for Perm
where
    F: Field,
{
    type Output = ColVec<F>;

    fn mul(self, other: &ColVec<F>) -> Self::Output {
        &self * other
    }
}

impl<F> Mul<ColVec<F>> for &Perm
where
    F: Field,
{
    type Output = ColVec<F>;

    fn mul(self, other: ColVec<F>) -> Self::Output {
        self * &other
    }
}

impl<F> Mul<&ColVec<F>> for &Perm
where
    F: Field,
{
    type Output = ColVec<F>;

    fn mul(self, vec: &ColVec<F>) -> Self::Output {
        let mut res = ColVec::zero(vec.field(), vec.rows());
        for i in 0..self.len() {
            res[self[i]] = vec[i];
        }
        res
    }
}
