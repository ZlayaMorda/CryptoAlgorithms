//! Matrices on a field

use std::rc::Rc;

use crate::finite_field::{F2FiniteExtension, Field, F2};

pub use colvec::ColVec;
pub use perm::Perm;
pub use rowvec::RowVec;
pub use submat::SubMat;

/// Matrix with coefficients in a field F
#[derive(Eq, PartialEq)]
pub struct Mat<F>
where
    F: Field,
{
    field: Rc<F>,
    rows: usize,
    cols: usize,
    data: Vec<F::FieldElement>,
}

impl<F> Mat<F>
where
    F: Field,
{
    /// Creates a new matrix
    ///
    /// The vector data holds the matrix coefficients, row by row.
    ///
    /// # Panics
    ///
    /// Panics if the matrix is empty or if there are not exactly rows * cols coefficients.
    pub fn new(field: Rc<F>, rows: usize, cols: usize, data: Vec<F::FieldElement>) -> Self {
        if rows == 0 || cols == 0 || data.is_empty() {
            panic!("Empty matrix");
        } else if data.len() != rows * cols {
            panic!("Dimensions do not match the number of coefficients");
        }
        Self {
            field,
            rows,
            cols,
            data,
        }
    }

    /// Creates a new matrix whose coefficients are all zero
    ///
    /// # Panics
    ///
    /// Panics if the number of either rows or columns is zero.
    pub fn zero(field: Rc<F>, rows: usize, cols: usize) -> Self {
        if rows == 0 || cols == 0 {
            panic!("Empty matrix");
        }
        let data = vec![field.zero(); rows * cols];
        Self {
            field,
            rows,
            cols,
            data,
        }
    }

    pub fn copy(&mut self, original: &Self) {
        self.data.copy_from_slice(original.data.as_slice());
    }

    pub fn copy_identity(&mut self) {
        self.fill(self.field.zero());
        for i in 0..self.rows {
            self[(i, i)] = self.field.one();
        }
    }

    pub fn fill(&mut self, x: F::FieldElement) {
        for i in &mut self.data {
            *i = x;
        }
    }

    pub fn field(&self) -> Rc<F> {
        Rc::clone(&self.field)
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn data(&self) -> &Vec<F::FieldElement> {
        &self.data
    }

    pub fn random(field: Rc<F>, n: usize, m: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut mat = Self::zero(field, n, m);
        for i in 0..n {
            for j in 0..m {
                mat[(i, j)] = mat.field.random_element(&mut rng);
            }
        }
        mat
    }

    pub fn is_zero(&self) -> bool {
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self[(i, j)] != self.field.zero() {
                    return false;
                }
            }
        }
        true
    }

    /// Creates a new matrix by taking the chosen rows in the given order
    pub fn extract_rows(&self, rows: &Vec<usize>) -> Self {
        let mut res = Mat::zero(self.field(), rows.len(), self.cols);
        for i in 0..rows.len() {
            for j in 0..self.cols {
                res[(i, j)] = self[(rows[i], j)];
            }
        }
        res
    }

    /// Creates a new matrix by taking the chosen columns in the given order
    pub fn extract_cols(&self, cols: &Vec<usize>) -> Self {
        let mut res = Mat::zero(self.field(), self.rows, cols.len());
        for j in 0..cols.len() {
            for i in 0..res.rows {
                res[(i, j)] = self[(i, cols[j])];
            }
        }
        res
    }

    pub fn identity(field: Rc<F>, n: usize) -> Self {
        let mut id = Self::zero(field, n, n);
        for i in 0..n {
            id[(i, i)] = id.field.one();
        }
        id
    }

    pub fn transpose(&self) -> Self {
        let mut t = Self::zero(self.field(), self.cols, self.rows);
        for i in 0..t.rows {
            for j in 0..t.cols {
                t[(i, j)] = self[(j, i)];
            }
        }
        t
    }

    /// Keep only given rows
    ///
    /// # Panics
    ///
    /// Panics if the vector contains an index that does not match any row of the matrix
    pub fn keep_rows(&mut self, rows: &Vec<usize>) {
        let mut rrows = rows.clone();
        rrows.sort_by(|a, b| a.cmp(b).reverse());
        if *rrows.first().unwrap() >= self.rows {
            panic!("invalid row index");
        }
        let mut iter = rrows.into_iter();
        let mut row = iter.next().unwrap_or(self.rows);
        for i in (0..self.rows).rev() {
            if i == row {
                row = iter.next().unwrap_or(self.rows);
                continue;
            }
            self.data.drain(i * self.cols..(i + 1) * self.cols);
            self.rows -= 1;
        }
    }

    /// Remove the given rows from the matrix
    ///
    /// # Panics
    ///
    /// Panics if the vector contains an index that does not match any row of the matrix
    pub fn remove_rows(&mut self, rows: &Vec<usize>) {
        let mut rrows = rows.clone();
        rrows.sort_by(|a, b| a.cmp(b).reverse());
        if *rrows.first().unwrap() >= self.rows {
            panic!("invalid row index");
        }
        for i in rrows.iter() {
            self.data.drain(i * self.cols..(i + 1) * self.cols);
            self.rows -= 1;
        }
    }

    pub fn mul(&mut self, a: &Self, b: &Self) {
        if self.field != a.field || a.field != b.field {
            panic!("Cannot multiply matrices: fields don't match");
        } else if self.rows != a.rows || a.cols != b.rows || b.cols != self.cols {
            panic!("Cannot multiply matrices: dimensions don't match");
        }

        let f = a.field();
        for i in 0..self.rows {
            for j in 0..self.cols {
                self[(i, j)] = f.zero();
                for k in 0..a.cols {
                    self[(i, j)] = f.add(self[(i, j)], f.mul(a[(i, k)], b[(k, j)]));
                }
            }
        }
    }

    /// Takes a t * n matrix on F<sub>2<sup>m</sup></sub>
    /// and outputs a mt * n matrix on F<sub>2</sub>
    /// by decomposing each coefficient on the canonical basis
    pub fn binary(&self, field: Rc<F2>) -> Mat<F2>
    where
        F: F2FiniteExtension,
    {
        let f = self.field();
        let m = f.characteristic_exponent() as usize;
        let mut bin = Mat::zero(field, m * self.rows, self.cols);
        for j in 0..self.cols {
            for i in 0..self.rows {
                let elt_as_u32 = f.elt_to_u32(self[(i, j)]);
                for k in 0..m {
                    bin[(m * i + k, j)] = (elt_as_u32 >> k) & 1;
                }
            }
        }
        bin
    }

    pub fn hconcat(a: &Mat<F>, b: &Mat<F>) -> Mat<F> {
        if a.rows != b.rows {
            panic!("number of rows do no match");
        }
        let mut ab = Mat::zero(a.field(), a.rows, a.cols + b.cols);
        for i in 0..ab.rows {
            for j in 0..a.cols {
                ab[(i, j)] = a[(i, j)];
            }
            for j in 0..b.cols {
                ab[(i, a.cols + j)] = b[(i, j)];
            }
        }
        ab
    }

    pub fn vconcat(a: &Mat<F>, b: &Mat<F>) -> Mat<F> {
        if a.cols != b.cols {
            panic!("number of columns do not match");
        }
        let mut ab = Mat::zero(a.field(), a.rows + b.rows, a.cols);
        for j in 0..ab.cols {
            for i in 0..a.rows {
                ab[(i, j)] = a[(i, j)];
            }
            for i in 0..b.rows {
                ab[(a.rows + i, j)] = b[(i, j)];
            }
        }
        ab
    }

    pub fn random_standard_form_parity_check_matrix(field: Rc<F>, n: usize, k: usize) -> Self {
        assert!(k <= n, "k must be at most n");
        let mut rng = rand::thread_rng();
        let mut h = Self::zero(field, n - k, n);
        for i in 0..n - k {
            h[(i, k + i)] = h.field.one();
            for j in 0..k {
                h[(i, j)] = h.field.random_element(&mut rng);
            }
        }
        h
    }
}

pub mod colvec;
pub mod gauss;
pub mod io;
pub mod perm;
pub mod rowvec;
pub mod submat;
pub mod traits;
