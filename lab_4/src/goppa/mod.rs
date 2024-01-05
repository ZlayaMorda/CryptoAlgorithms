//! Binary irreducible Goppa codes

use log::debug;
use rand::Rng;
use std::{
    fmt::{Debug, Display, Formatter, Result},
    rc::Rc,
};

use crate::{
    finite_field::{F2FiniteExtension, Field, F2},
    matrix::{Mat, RowVec},
    polynomial::Poly,
};

/// Binary irreducible Goppa code
#[derive(Eq, PartialEq)]
pub struct Goppa<F>
where
    F: Field,
{
    poly: Poly<F>,
    set: Vec<F::FieldElement>,
}

impl<F> Debug for Goppa<F>
where
    F: F2FiniteExtension,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let fq = self.poly.field();
        let q = fq.order();
        let n = self.set.len();
        let t = self.poly.degree();
        write!(
            f,
            "Goppa code (L, g(x)) from F{} with n={} and t={}\n",
            q, n, t
        )?;
        write!(f, "g(x) = {:?}\n", self.poly)?;
        if n == q {
            write!(f, "L = F{}\n", q)
        } else {
            write!(f, "L = [")?;
            for i in 0..n - 1 {
                write!(f, "{:X}, ", fq.elt_to_u32(self.set[i]))?;
            }
            write!(f, "{:X}]\n", fq.elt_to_u32(self.set[n - 1]))
        }
    }
}

impl<F> Display for Goppa<F>
where
    F: F2FiniteExtension,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let fq = self.poly.field();
        let q = fq.order();
        let n = self.set.len();
        let t = self.poly.degree();
        write!(
            f,
            "Goppa code (L, g(x)) from F{} with n={} and t={}\n",
            q, n, t
        )?;
        write!(f, "g(x) = {}\n", self.poly)?;
        if n == q {
            write!(f, "L = F{}\n", q)
        } else {
            write!(f, "L = [")?;
            for i in 0..n - 1 {
                write!(f, "{}, ", fq.elt_to_str(self.set[i]))?;
            }
            write!(f, "{}]\n", fq.elt_to_str(self.set[n - 1]))
        }
    }
}

impl<F> Goppa<F>
where
    F: F2FiniteExtension,
{
    /// Creates a binary irreducible Goppa code
    ///
    /// poly must be an irreducible polynomial on F
    /// and set a subset of F which does not contain any root of the polynomial.  
    /// Set elements must be ordered according to their u32 representation
    /// (given by function `elt_to_u32()`).
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - poly is not irreducible
    /// - set contains twice the same element
    /// - set contains a root of the polynomial
    /// - set elements are not ordered according to their u32 representation
    pub fn new(poly: Poly<F>, set: Vec<F::FieldElement>) -> Self {
        if !poly.is_irreducible() {
            panic!("Goppa polynomial is not irreducible");
        }
        let f = poly.field();
        for i in 0..set.len() - 1 {
            if f.elt_to_u32(set[i]) == f.elt_to_u32(set[i + 1]) {
                panic!("Set elements must be different");
            }
            if f.elt_to_u32(set[i]) > f.elt_to_u32(set[i + 1]) {
                panic!("Set elements must be ordered according to their u32 representation");
            }
        }
        if poly.degree() == 1 && set.contains(&f.mul(f.inv(poly[1]).unwrap(), poly[0])) {
            panic!("Set contains a root of the Goppa polynomial");
        }
        Self { poly, set }
    }

    /// Generates from field F a random binary irreducible Goppa code
    /// of length n and of correction capacity t
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - n is greater than the order of F
    /// - n &le; t * log<sub>2</sub>|F| (Goppa code dimension would be 0)
    /// - n = log<sub>2</sub>|F| and t = 1 (a Goppa code set cannot contain one of its roots)
    pub fn random(field: Rc<F>, n: usize, t: usize) -> Self {
        let poly = Poly::random_monic_irreducible(field, t);
        let f = poly.field();
        let q = f.order();
        let m = f.characteristic_exponent() as usize;

        if n > q {
            panic!("n must be at most q");
        }
        if poly.degree() == 1 && n == q {
            panic!("n must be strictly less than q when Goppa polynomial is of degree 1");
        }
        if n <= m * t {
            panic!("m * t must be at most n");
        }

        let mut pool = Vec::with_capacity(q);
        for i in 0..q as u32 {
            pool.push(i);
        }
        if poly.degree() == 1 {
            let root = f.mul(f.inv(poly[1]).unwrap(), poly[0]);
            pool.swap_remove(f.elt_to_u32(root) as usize);
        }
        let mut rng = rand::thread_rng();
        let mut set = Vec::with_capacity(n);
        for _i in 0..n {
            let index = rng.gen_range(0, pool.len());
            set.push(pool[index]);
            pool.swap_remove(index);
        }
        set.sort();
        let set = set.iter().map(|x| f.u32_to_elt(*x)).collect();
        Self { poly, set }
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn poly(&self) -> &Poly<F> {
        &self.poly
    }

    pub fn set(&self) -> &Vec<F::FieldElement> {
        &self.set
    }

    pub fn field(&self) -> Rc<F> {
        Rc::clone(&self.poly.field())
    }

    pub fn parity_check_x(&self) -> Mat<F> {
        let f = self.field();
        let t = self.poly.degree();
        let mut x = Mat::zero(f, t, t);
        for i in 0..t {
            for j in 0..i + 1 {
                x[(i, j)] = self.poly[t - i + j];
            }
        }
        x
    }

    pub fn parity_check_y(&self) -> Mat<F> {
        let f = self.field();
        let n = self.len();
        let t = self.poly.degree();

        let mut y = Mat::zero(Rc::clone(&f), t, n);
        for i in 0..n {
            y[(0, i)] = f.one();
        }
        for i in 1..t {
            for j in 0..n {
                let elt = self.set[j];
                y[(i, j)] = f.mul(elt, y[(i - 1, j)]);
            }
        }
        y
    }

    pub fn parity_check_z(&self) -> Mat<F> {
        let f = self.field();
        let n = self.len();

        let mut z = Mat::zero(Rc::clone(&f), n, n);
        for i in 0..n {
            z[(i, i)] = f.inv(self.poly.eval(self.set[i])).unwrap();
        }
        z
    }

    pub fn parity_check_xyz(&self) -> Mat<F> {
        let x = self.parity_check_x();
        let y = self.parity_check_y();
        let z = self.parity_check_z();
        x * y * z
    }

    pub fn parity_check_from_xyz(xyz: &Mat<F>, f2: Rc<F2>) -> Mat<F2> {
        let mut h = xyz.binary(f2);
        h.remove_redundant_rows();
        h
    }

    pub fn parity_check_matrix(&self, f2: Rc<F2>) -> Mat<F2> {
        let xyz = self.parity_check_xyz();
        Self::parity_check_from_xyz(&xyz, f2)
    }

    pub fn generator_from_xyz(xyz: &Mat<F>, f2: Rc<F2>) -> (Mat<F2>, Vec<usize>) {
        let xyz2 = xyz.binary(f2);
        let (hs, p) = xyz2.standard_parity_check_equivalent();
        let gs = Self::generator_from_parity_check_standard(&hs);
        let k = hs.cols() - hs.rows();
        let information_set = p.data()[0..k].to_vec();
        (gs * p.inverse(), information_set)
    }

    pub fn generator_from_parity_check_standard(h: &Mat<F2>) -> Mat<F2> {
        let f2 = h.field();
        let n = h.cols();
        let k = n - h.rows();
        let mut g = Mat::zero(Rc::clone(&f2), k, n);
        for i in 0..k {
            g[(i, i)] = f2.one();
            for j in k..n {
                g[(i, j)] = f2.neg(h[(j - k, i)]);
            }
        }
        g
    }

    pub fn generator_from_parity_check(h: &Mat<F2>) -> (Mat<F2>, Vec<usize>) {
        let n = h.cols();
        let k = n - h.rows();
        let (hs, p) = h.standard_parity_check_equivalent();
        let gs = Goppa::<F>::generator_from_parity_check_standard(&hs);
        let information_set = p.data()[0..k].to_vec();
        (gs * p.inverse(), information_set)
    }

    pub fn generator_matrix(&self, f2: Rc<F2>) -> Mat<F2> {
        let xyz = self.parity_check_xyz();
        Self::generator_from_xyz(&xyz, f2).0
    }

    pub fn syndrome(&self, r: &RowVec<F2>) -> Mat<F> {
        let xyz = self.parity_check_xyz();
        Self::syndrome_from_xyz(&xyz, r)
    }

    pub fn syndrome_from_xyz(xyz: &Mat<F>, rcv: &RowVec<F2>) -> Mat<F> {
        let f2 = rcv.field();
        let f = xyz.field();
        let mut s = Mat::zero(Rc::clone(&f), xyz.rows(), 1);
        for i in 0..xyz.rows() {
            for j in 0..rcv.cols() {
                if rcv[j] == f2.one() {
                    s[(i, 0)] = f.add(s[(i, 0)], xyz[(i, j)]);
                }
            }
        }
        s
    }

    pub fn encode(&self, msg: &RowVec<F2>) -> RowVec<F2> {
        let f2 = msg.field();
        let g = self.generator_matrix(f2);
        Self::g_encode(&g, msg)
    }

    pub fn g_encode(g: &Mat<F2>, msg: &RowVec<F2>) -> RowVec<F2> {
        msg * g
    }

    pub fn decode(&self, rcv: &RowVec<F2>) -> RowVec<F2> {
        let xyz = self.parity_check_xyz();
        self.xyz_decode(&xyz, rcv)
    }

    pub fn xyz_decode(&self, xyz: &Mat<F>, rcv: &RowVec<F2>) -> RowVec<F2> {
        let f = self.field();
        let f2 = rcv.field();
        let syndrome = Self::syndrome_from_xyz(xyz, rcv);
        debug!("syndrome:{}", syndrome);

        let s_x = Poly::new(
            Rc::clone(&f),
            syndrome.data().iter().rev().cloned().collect(),
        );
        debug!("S(x) = {}", s_x);

        if s_x.is_zero() {
            return rcv.clone();
        }

        let mut t_x = s_x.inverse_modulo(&self.poly);
        debug!("T(x) = s(x)^-1 = {}", s_x);

        t_x += Poly::x_n(Rc::clone(&f), 1);
        t_x.square_root_modulo(&self.poly);
        debug!("(T(x) + x)^(1/2) = {}", t_x);

        let (mut a, mut b) = Poly::goppa_extended_gcd(&self.poly, &t_x);
        debug!("a(x) = {}", a);
        debug!("b(x) = {}", b);

        a.square();
        b.square();
        debug!("a(x)^2 = {}", a);
        debug!("b(x)^2 = {}", b);

        b *= Poly::x_n(Rc::clone(&f), 1);
        let sigma = a + b;
        debug!("sigma(x) = {}", sigma);

        let err = RowVec::new(
            Rc::clone(&f2),
            self.set
                .iter()
                .map(|x| {
                    if sigma.eval(*x) == f.zero() {
                        f2.one()
                    } else {
                        f2.zero()
                    }
                })
                .collect(),
        );
        debug!("Error vector:{}", err);

        let cdw = rcv + err;
        cdw
    }
}

pub mod io;
