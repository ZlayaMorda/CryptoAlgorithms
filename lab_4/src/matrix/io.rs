//! Converts a matrix on F<sub>2</sub> into a byte vector and vice versa

use std::{convert::TryInto, error::Error, rc::Rc};

use super::Mat;
use crate::finite_field::{Field, F2};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

impl Mat<F2> {
    /// Encodes the matrix in bytes
    ///
    /// We start by encoding numbers of rows and columns on four bytes each.
    /// The matrix data follows.
    pub fn to_bytes(&self) -> Vec<u8> {
        let len = 4 + 4 + crate::div_ceil(self.rows * self.cols, 8);
        let mut vec = Vec::with_capacity(len);
        vec.extend_from_slice(&(self.rows as u32).to_be_bytes());
        vec.extend_from_slice(&(self.cols as u32).to_be_bytes());
        let mut byte = 0;
        let mut shift = 7;
        for i in 0..self.rows {
            for j in 0..self.cols {
                byte |= (self[(i, j)] as u8) << shift;
                if shift == 0 {
                    vec.push(byte);
                    byte = 0;
                    shift = 7;
                } else {
                    shift -= 1;
                }
            }
        }
        if shift != 7 {
            vec.push(byte);
        }
        vec
    }

    /// Decodes bytes encoded with [`to_bytes()`] to a matrix
    ///
    /// [`to_bytes()`]: #method.to_bytes
    pub fn from_bytes(vec: &[u8]) -> Result<(usize, Self)> {
        let rows = u32::from_be_bytes(vec[0..4].try_into()?) as usize;
        let cols = u32::from_be_bytes(vec[4..8].try_into()?) as usize;
        let f2 = Rc::new(F2::generate(()));
        let mut mat = Mat::zero(f2, rows, cols);
        let mut read = 8;
        let mut shift = 7;
        for i in 0..rows {
            for j in 0..cols {
                mat[(i, j)] = ((vec[read] >> shift) & 1).into();
                if shift == 0 {
                    read += 1;
                    shift = 7;
                } else {
                    shift -= 1;
                }
            }
        }
        if shift != 7 {
            read += 1;
        }
        Ok((read, mat))
    }
}
