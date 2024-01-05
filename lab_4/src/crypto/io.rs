//! Reads and writes keys

use log::debug;
use std::{
    convert::TryInto,
    error::Error,
    fs::File,
    io::{BufWriter, Read, Write},
};

use super::{PublicKey, SecretKey};
use crate::{
    finite_field::FiniteField,
    goppa::Goppa,
    matrix::{Mat, Perm},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

impl PublicKey {
    /// Saves public key on disk
    ///
    /// The output file layout is:
    /// - bytes 0-3: number k of rows of matrix sgp
    /// - bytes 4-7: number n of columns of matrix sgp
    /// - bytes 8-x: coefficients of matrix sgp (eight per byte)
    /// - bytes x-x+4: correction capacity t
    pub fn write(&self, file_name: &str) -> Result<()> {
        let mut f = File::create(file_name)?;
        f.write_all(&self.sgp.to_bytes())?;
        f.write_all(&(self.t as u32).to_be_bytes())?;
        Ok(())
    }

    pub fn read_public_key(file_name: &str) -> Result<Self> {
        let mut f = File::open(file_name)?;
        let mut vec = Vec::new();
        f.read_to_end(&mut vec)?;
        let (i, sgp) = Mat::from_bytes(&vec)?;
        let t = u32::from_be_bytes(vec[i..i + 4].try_into()?) as usize;
        Ok(PublicKey { sgp, t })
    }

    pub fn read_code_dimension(file_name: &str) -> Result<usize> {
        let mut f = File::open(file_name)?;
        let mut buf = [0; 4];
        f.read_exact(&mut buf)?;
        let k = u32::from_be_bytes(buf) as usize;
        Ok(k)
    }
}

impl SecretKey {
    /// Saves secret key on disk
    ///
    /// The output file layout is:
    /// - bytes 0-3: finite field order q
    /// - bytes 4-7: number of rows of matrix s
    /// - bytes 8-11: number of columns of matrix s
    /// - bytes 11-a: coefficients of matrix s (eight per byte)
    /// - bytes a-a+4: finite field order q
    /// - bytes a+4-a+8: correction capacity t
    /// - bytes a+8-b: coefficients of Goppa polynomial (four bytes per coefficient)
    /// - bytes b-c: Goppa set L (see [`Goppa`]::[`to_bytes()`] for more information)
    /// - bytes c-d: information set (four bytes per column index)
    /// - bytes d-d+4: length n of the code
    /// - bytes d+4-e: permutation P (four bytes per image)
    ///
    /// [`Goppa`]: ../goppa/struct.Goppa.html
    /// [`to_bytes()`]: ../goppa/struct.Goppa.html#method.to_bytes
    pub fn write(&self, file_name: &str) -> Result<()> {
        let f = File::create(file_name)?;
        let mut f = BufWriter::new(f);
        f.write_all(&(self.goppa.field().order() as u32).to_be_bytes())?;
        f.write_all(&self.s.to_bytes())?;
        f.write_all(&self.goppa.to_bytes())?;

        for i in 0..self.info_set.len() {
            f.write_all(&(self.info_set[i] as u32).to_be_bytes())?;
        }

        f.write_all(&(self.p.len() as u32).to_be_bytes())?;
        for i in 0..self.p.len() {
            f.write_all(&(self.p[i] as u32).to_be_bytes())?;
        }
        Ok(())
    }

    pub fn read_secret_key(file_name: &str) -> Result<Self> {
        let mut f = File::open(file_name)?;
        let mut vec = Vec::new();
        f.read_to_end(&mut vec)?;
        let mut i = 4;

        let (bytes, s) = Mat::from_bytes(&vec[i..])?;
        i += bytes;
        debug!("Read matrix s:{}", s);

        let (bytes, goppa) = Goppa::from_bytes(&vec[i..])?;
        i += bytes;
        debug!("Read Goppa code:\n{}", goppa);

        let k = s.rows();
        let mut info_set = Vec::new();
        for _j in 0..k {
            info_set.push(u32::from_be_bytes(vec[i..i + 4].try_into()?) as usize);
            i += 4;
        }
        debug!("Read information set:\n{:?}", info_set);

        let p_len = u32::from_be_bytes(vec[i..i + 4].try_into()?) as usize;
        i += 4;
        let mut p = Vec::with_capacity(p_len);
        for _j in 0..p_len {
            p.push(u32::from_be_bytes(vec[i..i + 4].try_into()?) as usize);
            i += 4;
        }
        let p = Perm::new(p);

        Ok(SecretKey {
            s,
            goppa,
            info_set,
            p,
        })
    }
}
