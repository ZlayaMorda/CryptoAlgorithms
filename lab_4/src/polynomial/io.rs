//! Converts a polynomial on F<sub>2<sup>m</sup></sub> into a byte vector and vice versa

use std::{convert::TryInto, error::Error, rc::Rc};

use super::Poly;
use crate::finite_field::{F2FiniteExtension, F2m, Field, FiniteField};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

impl Poly<F2m> {
    /// Encodes the polynomial in bytes
    ///
    /// We start by encoding field order and degree of the polynomial followed by coefficients.
    /// Each number is encoded on four bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let f = self.field();
        let len = 4 + 4 + 4 * (self.degree() + 1);
        let mut vec = Vec::with_capacity(len);
        vec.extend_from_slice(&(f.order() as u32).to_be_bytes());
        vec.extend_from_slice(&(self.degree() as u32).to_be_bytes());
        for i in 0..self.degree() + 1 {
            vec.extend_from_slice(&(f.elt_to_u32(self[i])).to_be_bytes());
        }
        vec
    }

    pub fn from_bytes(vec: &[u8]) -> Result<(usize, Self)> {
        let order = u32::from_be_bytes(vec[0..4].try_into()?) as usize;
        let t = u32::from_be_bytes(vec[4..8].try_into()?) as usize;
        let f2m = Rc::new(F2m::generate(order));
        let mut poly = Self::zero(f2m, t + 1);
        for i in 0..t + 1 {
            let j = 8 + 4 * i;
            poly[i] = poly
                .field()
                .u32_to_elt(u32::from_be_bytes(vec[j..j + 4].try_into()?));
        }
        let read = 4 + 4 + 4 * (t + 1);
        Ok((read, poly))
    }
}
