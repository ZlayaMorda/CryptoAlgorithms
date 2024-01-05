use rand::Rng;
use std::rc::Rc;

use super::{ColVec, Mat, Perm};
use crate::finite_field::{Field};

#[derive(Eq, PartialEq)]
pub struct RowVec<F>(pub Mat<F>)
where
    F: Field;

impl<F> RowVec<F>
where
    F: Field,
{
    pub fn new(field: Rc<F>, data: Vec<F::FieldElement>) -> Self {
        RowVec(Mat::new(field, 1, data.len(), data))
    }

    pub fn zero(field: Rc<F>, cols: usize) -> Self {
        RowVec(Mat::zero(field, 1, cols))
    }

    pub fn field(&self) -> Rc<F> {
        self.0.field()
    }

    pub fn rows(&self) -> usize {
        1
    }

    pub fn cols(&self) -> usize {
        self.0.cols()
    }

    pub fn data(&self) -> &Vec<F::FieldElement> {
        self.0.data()
    }

    pub fn weight(&self) -> usize {
        let mut weight = 0;
        for j in 0..self.cols() {
            if self[j] != self.0.field.zero() {
                weight += 1;
            }
        }
        weight
    }

    pub fn random(field: Rc<F>, n: usize) -> Self {
        RowVec(Mat::random(field, 1, n))
    }

    pub fn random_with_weight(field: Rc<F>, n: usize, w: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut vec = RowVec::zero(field, n);
        let mut cols = Vec::with_capacity(n);
        for i in 0..n {
            cols.push(i);
        }

        for _i in 0..w {
            let mut elt;
            loop {
                elt = vec.field().random_element(&mut rng);
                if elt != vec.field().zero() {
                    break;
                }
            }
            let index = rng.gen_range(0, cols.len());
            vec[cols[index]] = elt;
            cols.swap_remove(index);
        }
        vec
    }

    pub fn transpose(&self) -> ColVec<F> {
        ColVec(self.0.transpose())
    }

    pub fn is_zero(&self) -> bool {
        for i in 0..self.cols() {
            if self[i] != self.field().zero() {
                return false;
            }
        }
        true
    }

    pub fn extract_cols(&self, perm: &Vec<usize>) -> Self {
        RowVec(self.0.extract_cols(perm))
    }
}

pub mod io;
pub mod traits;
