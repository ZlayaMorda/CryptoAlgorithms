//! Converts a Goppa code on F<sub>2<sup>m</sup></sub> into a byte vector and vice versa

use log::debug;
use std::{error::Error, result};

use super::Goppa;
use crate::{
    finite_field::{F2FiniteExtension, F2m, FiniteField},
    polynomial::Poly,
};

type Result<T> = result::Result<T, Box<dyn Error>>;

impl Goppa<F2m> {
    /// Encodes the Goppa code
    ///
    /// The encoded byte vector has the following layout:
    /// - bytes 0-3: finite field order q
    /// - bytes 4-7: degree of the Goppa polynomial t
    /// - bytes 8-a: coefficients of the Goppa polynomial (four bytes each)
    /// - bytes a-b: Goppa set L seen as a sequence of q bits
    ///   (the ith bit is set if [`u32_to_elt(i)`] belongs to L)
    ///
    /// [`u32_to_elt(i)`]: #tymethod.u32_to_elt
    pub fn to_bytes(&self) -> Vec<u8> {
        let f = self.field();
        let mut vec = self.poly.to_bytes();
        vec.reserve_exact(vec.len() + crate::div_ceil(f.order(), 8));
        let mut byte = 0;
        let mut shift = 7;
        let mut j = 0;
        for i in 0..f.order() as u32 {
            if Some(&f.u32_to_elt(i)) == self.set.get(j) {
                byte |= 1 << shift;
                j += 1;
            }
            if shift == 0 {
                vec.push(byte);
                byte = 0;
                shift = 7;
            } else {
                shift -= 1;
            }
        }
        if shift != 7 {
            vec.push(byte);
        }
        vec
    }

    pub fn from_bytes(vec: &[u8]) -> Result<(usize, Self)> {
        let (mut read, poly) = Poly::from_bytes(vec)?;
        debug!("Read polynomial:\n{}", poly);

        let f = poly.field();
        let mut set = Vec::new();
        let mut shift = 7;
        for j in 0..f.order() as u32 {
            if (vec[read] >> shift) & 1 == 1 {
                set.push(f.u32_to_elt(j));
            }
            if shift == 0 {
                shift = 7;
                read += 1;
            } else {
                shift -= 1
            }
        }
        if shift != 7 {
            read += 1;
        }
        Ok((read, Goppa::new(poly, set)))
    }
}
