//! Converts a row vector on F<sub>2</sub> into a byte vector and vice versa

use std::{
    convert::TryInto,
    error::Error,
    fs::File,
    io::{Read, Write},
    rc::Rc,
};
use log::debug;

use super::RowVec;
use crate::finite_field::{Field, F2};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

impl RowVec<F2> {
    pub fn write(&self, file_name: &str) -> Result<()> {
        let mut f = File::create(file_name)?;
        let len = crate::div_ceil(self.cols(), 8); // 4 + crate::div_ceil(self.cols(), 8);
        let mut vec = Vec::with_capacity(len);
        //vec.extend_from_slice(&(self.cols() as u32).to_be_bytes());
        let mut byte = 0;
        let mut shift = 7;
        for i in 0..self.cols() {
            byte |= (self[i] as u8) << shift;
            if shift == 0 {
                vec.push(byte);
                byte = 0;
                shift = 7;
            } else {
                shift -= 1;
            }
        }
        if self.cols() % 8 != 0 {
            vec.push(byte);
        }
        f.write_all(&vec)?;
        Ok(())
    }

    pub fn read_vector(file_name: &str, cols: usize) -> Result<RowVec<F2>> {
        let mut f = File::open(file_name)?;
        let mut vec = Vec::new();
        f.read_to_end(&mut vec)?;
        println!("len of u8 vec: {}", vec.len());
        // let cols = vec.len() * 8; // u32::from_be_bytes(vec[0..4].try_into()?) as usize;
        //     match cols {
        //     0 => { u32::from_be_bytes(vec[0..4].try_into()?) as usize }
        //     value => { value }
        // };
        println!("{cols}");
        let f2 = Rc::new(F2::generate(()));
        let mut rowvec = RowVec::zero(f2, cols);
        println!("row vec cols: {}", rowvec.cols());
        let mut k = 0;
        let mut shift = 7;
        for i in 0..cols {
            if k >= vec.len() {
                rowvec[i] = ((0_u8 >> shift) & 1).into();
            }
            else {
                rowvec[i] = ((vec[k] >> shift) & 1).into();
            }

            if shift == 0 {
                k += 1;
                shift = 7;
            } else {
                shift -= 1;
            }
        }
        Ok(rowvec)
    }
}
