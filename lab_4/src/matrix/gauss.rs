use rand::Rng;
use std::rc::Rc;

use super::{Mat, Perm};
use crate::finite_field::Field;

impl<F> Mat<F>
where
    F: Field,
{
    pub fn row_echelon_form(&mut self) -> Vec<usize> {
        let f = Rc::clone(&self.field);
        let n = self.rows;
        let m = self.cols;
        let mut row_pivot = 0;
        let mut col_pivot = 0;
        let mut p = Perm::identity(n);
        let mut rank = 0;

        while row_pivot < self.rows && col_pivot < self.cols {
            // Find pivot
            let mut i = row_pivot;
            while i < n && self[(i, col_pivot)] == f.zero() {
                i += 1;
            }
            if i == n {
                col_pivot += 1;
                continue;
            }
            self.swap_rows(i, row_pivot);
            p.swap(i, row_pivot);
            rank += 1;

            // Normalize pivot's row
            let pivot_inv = f.inv(self[(row_pivot, col_pivot)]).unwrap();
            for j in col_pivot + 1..m {
                self[(row_pivot, j)] = f.mul(pivot_inv, self[(row_pivot, j)]);
            }
            self[(row_pivot, col_pivot)] = f.one();

            // Adjust all rows below pivot's row
            for k in row_pivot + 1..n {
                if self[(k, col_pivot)] == f.zero() {
                    continue;
                }
                for l in col_pivot + 1..m {
                    self[(k, l)] = f.sub(
                        self[(k, l)],
                        f.mul(self[(k, col_pivot)], self[(row_pivot, l)]),
                    );
                }
                self[(k, col_pivot)] = f.zero();
            }

            row_pivot += 1;
            col_pivot += 1;
        }
        p.data()[0..rank].to_vec()
    }

    pub fn reduced_row_echelon_form(&mut self) -> Vec<usize> {
        let f = Rc::clone(&self.field);
        let n = self.rows;
        let m = self.cols;
        let max_set_of_independant_rows = self.row_echelon_form(); // note that all pivots are 1

        for row_pivot in (0..n).rev() {
            // Find the pivot on this row if any
            let mut col_pivot = 0;
            while col_pivot < m && self[(row_pivot, col_pivot)] == f.zero() {
                col_pivot += 1;
            }
            if col_pivot == m {
                continue;
            }

            // Eliminate all non zero elements in the pivot's column
            for i in (0..row_pivot).rev() {
                if self[(i, col_pivot)] == f.zero() {
                    continue;
                }

                for k in col_pivot..m {
                    self[(i, k)] = f.sub(
                        self[(i, k)],
                        f.mul(self[(i, col_pivot)], self[(row_pivot, col_pivot)]),
                    );
                }
            }
        }
        max_set_of_independant_rows
    }

    /// Compute, if possible, (U, S, P) with U invertible, S standard form and P permutation
    /// such that S = U * self * P
    pub fn standard_form(&self) -> Option<(Self, Self, Perm)> {
        let f = self.field();
        let m = self.rows;
        let n = self.cols;
        if m > n {
            return None;
        }
        let mut u = Mat::identity(Rc::clone(&f), m);
        let mut h = self.clone();
        let mut p = Perm::identity(n);
        let mut col = n; // index of the column to check for a pivot

        // j is the index of the column to "standardize":
        // The first iteration sets a 1 at the last position (m-1) of column n-1.
        // The second iteration sets a 1 at position m-2 of column n-2.
        // ...
        // The last iteration sets a 1 at position 0 of column n-m.
        for j in (n - m..n).rev() {
            // Among the remaining columns, select one with a pivot
            let mut pivot = false;
            let mut row_pivot = 0;
            let mut col_pivot = 0;
            while !pivot && col != 0 {
                col -= 1;

                // Check column 'col' for a pivot
                for row in (0..j + m - n + 1).rev() {
                    if h[(row, col)] != f.zero() {
                        pivot = true;
                        row_pivot = row;
                        col_pivot = col;
                        break;
                    }
                }
            }

            if !pivot {
                return None;
            }

            // Put pivot column in the adequate position and update P
            h.swap_cols(j, col_pivot);
            p.swap(j, col_pivot);

            // Put pivot row in the adequate position and update U
            h.swap_rows(j + m - n, row_pivot);
            u.swap_rows(j + m - n, row_pivot);

            // Pivot is now at (j+m-n, j)

            // Multiply pivot row by pivot^-1 and update U
            let pivot_inv = f.inv(h[(j + m - n, j)]).unwrap();
            for k in 0..n {
                h[(j + m - n, k)] = f.mul(pivot_inv, h[(j + m - n, k)]);
            }
            for k in 0..m {
                u[(j + m - n, k)] = f.mul(pivot_inv, u[(j + m - n, k)]);
            }

            // Nullify the rest of the column and update matrix U accordingly
            for i in 0..m {
                if h[(i, j)] != f.zero() && i != j + m - n {
                    let lambda = f.neg(h[(i, j)]);
                    h.combine_rows(i, lambda, j + m - n);
                    u.combine_rows(i, lambda, j + m - n);
                }
            }
        }
        Some((u, h, p))
    }

    //TODO: can the random drawing of the information set be optimized ? Right now the same column can be picked and discarded multiple times.
    /// Like standard_form, except the information set is picked at random
    pub fn random_standard_form(&self) -> Option<(Self, Self, Perm)> {
        let f = self.field();
        let m = self.rows;
        let n = self.cols;
        if m > n {
            return None;
        }
        let mut rng = rand::thread_rng();
        let mut u = Mat::identity(Rc::clone(&f), m);
        let mut h = self.clone();
        let mut p = Perm::identity(n);
        let mut pivot_candidates = vec![0; n];

        // j is the index of the column to "standardize":
        // The first iteration sets a 1 at the last position (m-1) of column n-1.
        // The second iteration sets a 1 at position m-2 of column n-2.
        // ...
        // The last iteration sets a 1 at position 0 of column n-m.
        for j in (n - m..n).rev() {
            // Among the remaining columns, select one with a pivot
            let mut pivot = false;
            let mut row_pivot = 0;
            let mut col_pivot = 0;
            pivot_candidates.resize(j + 1, 0);
            for i in 0..j + 1 {
                pivot_candidates[i] = i;
            }

            while !pivot && !pivot_candidates.is_empty() {
                let index = rng.gen_range(0, pivot_candidates.len()); // index of the column to check for a pivot
                let col = pivot_candidates.swap_remove(index);

                // Check column 'col' for a pivot
                for row in (0..j + m - n + 1).rev() {
                    if h[(row, col)] != f.zero() {
                        pivot = true;
                        row_pivot = row;
                        col_pivot = col;
                        break;
                    }
                }
            }

            if !pivot {
                return None;
            }

            // Put pivot column in the adequate position and update P
            h.swap_cols(j, col_pivot);
            p.swap(j, col_pivot);

            // Put pivot row in the adequate position and update U
            h.swap_rows(j + m - n, row_pivot);
            u.swap_rows(j + m - n, row_pivot);

            // Pivot is now at (j+m-n, j)

            // Multiply pivot row by pivot^-1 and update U
            let pivot_inv = f.inv(h[(j + m - n, j)]).unwrap();
            for k in 0..n {
                h[(j + m - n, k)] = f.mul(pivot_inv, h[(j + m - n, k)]);
            }
            for k in 0..m {
                u[(j + m - n, k)] = f.mul(pivot_inv, u[(j + m - n, k)]);
            }

            // Nullify the rest of the column and update matrix U accordingly
            for i in 0..m {
                if h[(i, j)] != f.zero() && i != j + m - n {
                    let lambda = f.neg(h[(i, j)]);
                    h.combine_rows(i, lambda, j + m - n);
                    u.combine_rows(i, lambda, j + m - n);
                }
            }
        }
        Some((u, h, p))
    }

    pub fn parity_check_random_standard_form(&self, u: &mut Self, h: &mut Self, p: &mut Perm) {
        let f = self.field();
        let m = self.rows;
        let n = self.cols;
        if m > n {
            panic!("Parity-check matrix must have at least as many columns as rows")
        }
        let mut rng = rand::thread_rng();
        u.copy_identity();
        h.copy(self);
        p.copy_identity();

        // j is the index of the column to "standardize":
        // The first iteration sets a 1 at the last position (m-1) of column n-1.
        // The second iteration sets a 1 at position m-2 of column n-2.
        // ...
        // The last iteration sets a 1 at position 0 of column n-m.
        for j in (n - m..n).rev() {
            // Among the remaining columns, select one with a pivot
            let mut pivot = false;
            let mut row_pivot = 0;
            let mut col_pivot = 0;

            while !pivot {
                let col = rng.gen_range(0, j + 1); // index of the column to check for a pivot

                // Check column 'col' for a pivot
                for row in (0..j + m - n + 1).rev() {
                    if h[(row, col)] != f.zero() {
                        pivot = true;
                        row_pivot = row;
                        col_pivot = col;
                        break;
                    }
                }
            }

            // Put pivot column in the adequate position and update P
            h.swap_cols(j, col_pivot);
            p.swap(j, col_pivot);

            // Put pivot row in the adequate position and update U
            h.swap_rows(j + m - n, row_pivot);
            u.swap_rows(j + m - n, row_pivot);

            // Pivot is now at (j+m-n, j)

            // Multiply pivot row by pivot^-1 and update U
            let pivot_inv = f.inv(h[(j + m - n, j)]).unwrap();
            for k in 0..n {
                h[(j + m - n, k)] = f.mul(pivot_inv, h[(j + m - n, k)]);
            }
            for k in 0..m {
                u[(j + m - n, k)] = f.mul(pivot_inv, u[(j + m - n, k)]);
            }

            // Nullify the rest of the column and update matrix U accordingly
            for i in 0..m {
                if h[(i, j)] != f.zero() && i != j + m - n {
                    let lambda = f.neg(h[(i, j)]);
                    h.combine_rows(i, lambda, j + m - n);
                    u.combine_rows(i, lambda, j + m - n);
                }
            }
        }
    }

    /// Takes a parity-check matrix H possibly with redundant rows.
    /// Returns (S, P) with S standard form and P permutation such that
    /// SP<sup>-1</sup> is a (full rank) parity-check matrix of the code  
    /// (S defines an equivalent code).
    pub fn standard_parity_check_equivalent(&self) -> (Self, Perm) {
        let f = self.field();
        let m = self.rows;
        let n = self.cols;
        if m > n {
            panic!(
                "Matrix must have at most as many rows as columns for parity-check decomposition"
            );
        }
        let mut h = self.clone();
        let mut p = Perm::identity(n);
        let mut col = n; // index of the column to check for a pivot

        // j is the index of the column to "standardize":
        // The first iteration sets a 1 at the last position (m-1) of column n-1.
        // The second iteration sets a 1 at position m-2 of column n-2.
        // ...
        // The last iteration sets a 1 at position 0 of column n-m.
        for j in (n - m..n).rev() {
            // Among the remaining columns, select one with a pivot
            let mut pivot = false;
            let mut row_pivot = 0;
            let mut col_pivot = 0;
            while !pivot && col != 0 {
                col -= 1;

                // Check column 'col' for a pivot
                for row in (0..j + m - n + 1).rev() {
                    if h[(row, col)] != f.zero() {
                        pivot = true;
                        row_pivot = row;
                        col_pivot = col;
                        break;
                    }
                }
            }

            if !pivot {
                h.remove_rows(&(0..j - (n - m) + 1).collect());
                return (h, p);
            }

            // Put pivot column in the adequate position and update P
            h.swap_cols(j, col_pivot);
            p.swap(j, col_pivot);

            // Put pivot row in the adequate position and update U
            h.swap_rows(j + m - n, row_pivot);

            // Pivot is now at (j+m-n, j)

            // Multiply pivot row by pivot^-1 and update U
            let pivot_inv = f.inv(h[(j + m - n, j)]).unwrap();
            for k in 0..n {
                h[(j + m - n, k)] = f.mul(pivot_inv, h[(j + m - n, k)]);
            }

            // Nullify the rest of the column and update matrix U accordingly
            for i in 0..m {
                if h[(i, j)] != f.zero() && i != j + m - n {
                    let lambda = f.neg(h[(i, j)]);
                    h.combine_rows(i, lambda, j + m - n);
                }
            }
        }
        (h, p)
    }

    /// Inverse computation via gaussian elimination
    ///
    /// See <https://en.wikipedia.org/wiki/Gaussian_elimination>.
    pub fn inverse(&self) -> Option<Self> {
        if self.rows != self.cols {
            return None;
        }

        let f = self.field();
        let n = self.rows;
        let mut mat = self.clone();
        let mut inv: Self = Mat::identity(Rc::clone(&f), n);
        let mut p = 0; // pivot's row and pivot's column

        while p < n {
            // Find pivot
            let mut i = p;
            while i < n && mat[(i, p)] == f.zero() {
                i += 1;
            }
            if i == n {
                return None;
            }

            // Swap rows and mimic operation on matrix 'inv'
            mat.swap_rows(i, p);
            inv.swap_rows(i, p);

            // Normalize pivot's row: L(p) = pivot^-1 * L(p)
            let pivot_inv = f.inv(mat[(p, p)]).unwrap();
            for j in p + 1..n {
                // first p+1 columns are zero
                mat[(p, j)] = f.mul(pivot_inv, mat[(p, j)]);
            }
            mat[(p, p)] = f.one();

            // Mimic normalization on matrix 'inv'
            for j in 0..n {
                inv[(p, j)] = f.mul(pivot_inv, inv[(p, j)]);
            }

            // Adjust all rows below pivot's row: L(k) = L(k) - c(k,p) * L(p)
            // where L(k) is the kth row and c(k,p) is the coefficient [k,p] of our matrix
            for k in p + 1..n {
                if mat[(k, p)] == f.zero() {
                    continue;
                }

                let lambda = mat[(k, p)];
                mat[(k, p)] = f.zero();
                for l in p + 1..n {
                    // first p+1 columns are zero
                    mat[(k, l)] = f.sub(mat[(k, l)], f.mul(lambda, mat[(p, l)]));
                }

                // Mimic operation on matrix 'inv'
                for l in 0..n {
                    inv[(k, l)] = f.sub(inv[(k, l)], f.mul(lambda, inv[(p, l)]));
                }
            }

            p += 1;
        }

        // Matrix 'mat' is now in triangular form

        for j in (0..n).rev() {
            for i in (0..j).rev() {
                // Perform the row operation to set c(i, j) to 0:
                // L(i) = L(i) - c(i, j) * L(j)
                // We don't actually need to operate on the original matrix here.
                // Mimic the row operation on matrix 'inv'.
                if mat[(i, j)] == f.zero() {
                    continue;
                }

                for l in 0..n {
                    inv[(i, l)] = f.sub(inv[(i, l)], f.mul(mat[(i, j)], inv[(j, l)]));
                }
            }
        }
        Some(inv)
    }

    /// Generates a random invertible matrix
    ///
    /// First generates a random matrix then applies to it the standard form algorithm.
    /// Keeps track of the applied transformations via an invertible matrix u.
    /// Returns u as our random invertible matrix.
    pub fn invertible_random(f: Rc<F>, n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut mat = Mat::random(Rc::clone(&f), n, n);
        let mut u = Mat::identity(Rc::clone(&f), n);

        // Loop on columns
        for j in 0..n {
            // Find a pivot in column j
            let mut i = j;
            while i < n && mat[(i, j)] == f.zero() {
                i += 1;
            }

            // If column j has no pivot, create it
            if i == n {
                i = rng.gen_range(j, n);
                loop {
                    mat[(i, j)] = f.random_element(&mut rng);
                    if mat[(i, j)] != f.zero() {
                        break;
                    }
                }
            }

            // Put pivot in position (j, j) and mirror operation on matrix u
            mat.swap_rows(i, j);
            u.swap_rows(i, j);

            // Normalize pivot's row and mirror operation on matrix u
            let pivot = mat[(j, j)];
            let inv_pivot = f.inv(pivot).unwrap();
            mat.mul_row(j, inv_pivot);
            u.mul_row(j, inv_pivot);

            // Zero coefficients under the pivot and mirror operation on matrix u
            for i in j + 1..n {
                if mat[(i, j)] == f.zero() {
                    continue;
                }
                let lambda = mat[(i, j)];
                mat[(i, j)] = f.zero();
                for k in j + 1..n {
                    mat[(i, k)] = f.sub(mat[(i, k)], f.mul(lambda, mat[(j, k)]));
                }
                u.combine_rows(i, lambda, j);
            }
        }
        u
    }

    pub fn remove_redundant_rows(&mut self) {
        let mut clone = self.clone();
        let rows = clone.max_set_of_independant_rows();
        self.keep_rows(&rows);
    }

    pub fn max_set_of_independant_rows(&mut self) -> Vec<usize> {
        self.row_echelon_form()
    }

    pub fn rank(&self) -> usize {
        let mut mat = self.clone();
        mat.row_echelon_form().len()
    }

    pub fn is_invertible(&self) -> bool {
        self.rows == self.cols && self.rows == self.rank()
    }

    pub fn is_standard_form(&self) -> bool {
        let f = self.field();
        let m = self.rows;
        let n = self.cols;
        if m > n {
            return false;
        }
        for i in 0..m {
            for j in n - m..n {
                if n + i == m + j {
                    if self[(i, j)] != f.one() {
                        return false;
                    }
                } else {
                    if self[(i, j)] != f.zero() {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        if row1 == row2 {
            return;
        }
        let row1_start = row1 * self.cols;
        let row1_end = row1_start + self.cols;
        let row2_start = row2 * self.cols;
        let row2_end = row2_start + self.cols;
        let mut slice = self.data[row1_start..row1_end].to_vec();
        slice.swap_with_slice(&mut self.data[row2_start..row2_end]);
        slice.swap_with_slice(&mut self.data[row1_start..row1_end]);
    }

    pub fn swap_cols(&mut self, col1: usize, col2: usize) {
        if col1 == col2 {
            return;
        }
        for i in 0..self.rows {
            self.data.swap(i * self.cols + col1, i * self.cols + col2);
        }
    }

    pub fn mul_row(&mut self, row: usize, lambda: F::FieldElement) {
        for j in 0..self.cols {
            self[(row, j)] = self.field.mul(lambda, self[(row, j)]);
        }
    }

    /// Modifies row1 such that row1 = row1 + lambda * row2
    pub fn combine_rows(&mut self, row1: usize, lambda: F::FieldElement, row2: usize) {
        for j in 0..self.cols {
            self[(row1, j)] = self
                .field
                .add(self[(row1, j)], self.field.mul(lambda, self[(row2, j)]));
        }
    }
}
