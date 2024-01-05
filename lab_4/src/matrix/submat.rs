use super::Mat;
use std::{
    fmt::{Display, Formatter, Result},
    ops::Index,
    rc::Rc,
};

use crate::finite_field::{Field, FiniteField};

pub struct SubMat<'a, F>
where
    F: Field,
{
    mat: &'a Mat<F>,
    row0: usize,
    row1: usize,
    col0: usize,
    col1: usize,
}

impl<'a, F> SubMat<'a, F>
where
    F: Field,
{
    pub fn new(mat: &'a Mat<F>, row0: usize, row1: usize, col0: usize, col1: usize) -> Self {
        assert!(row0 < row1 && row1 <= mat.rows());
        assert!(col0 < col1 && col1 <= mat.cols());
        Self {
            mat,
            row0,
            row1,
            col0,
            col1,
        }
    }

    pub fn field(&self) -> Rc<F> {
        self.mat.field()
    }

    pub fn rows(&self) -> usize {
        self.row1 - self.row0
    }

    pub fn cols(&self) -> usize {
        self.col1 - self.col0
    }
}

impl<'a, F> Index<(usize, usize)> for SubMat<'a, F>
where
    F: Field,
{
    type Output = F::FieldElement;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let row = self.row0 + index.0;
        let col = self.col0 + index.1;
        &self.mat.data[row * self.mat.cols() + col]
    }
}

// impl<'a, F> IndexMut<(usize, usize)> for SubMat<'a, F>
// where
//     F: Field,
// {
//     fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
//         let row = self.row0 + index.0;
//         let col = self.col0 + index.1;
//         let cols = self.mat.cols();
//         &mut self.mat.data[row * cols + col]
//     }
// }

impl<'a, F> Display for SubMat<'a, F>
where
    F: FiniteField,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let k = self.field();

        // Upper bound on the number of digits of order
        let digits =
            ((usize::min_value().leading_zeros() - k.order().leading_zeros()) / 3 + 1) as usize;

        write!(f, "\n")?;
        for i in 0..self.rows() {
            for j in 0..self.cols() - 1 {
                write!(
                    f,
                    "{:>width$} ",
                    k.elt_to_str(self[(i, j)]),
                    width = if k.order() == 2 { 1 } else { 2 + digits }
                )?;
            }
            write!(
                f,
                "{:>width$}\n",
                k.elt_to_str(self[(i, self.cols() - 1)]),
                width = if k.order() == 2 { 1 } else { 2 + digits }
            )?;
        }
        Ok(())
    }
}
