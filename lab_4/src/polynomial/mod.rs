//! Polynomials on a field

use std::rc::Rc;

use crate::finite_field::Field;

/// Polynomial with coefficients in a field F
#[derive(Eq)]
pub struct Poly<F>
where
    F: Field,
{
    field: Rc<F>,
    data: Vec<F::FieldElement>,
}

impl<F> Poly<F>
where
    F: Field,
{
    /// Creates a new polynomial
    ///
    /// # Panics
    ///
    /// Panics if the polynomial is empty i.e. if the vector data is empty.
    pub fn new(field: Rc<F>, data: Vec<F::FieldElement>) -> Self {
        if data.is_empty() {
            panic!("Polynomial must have at least one coefficient");
        }
        Self { field, data }
    }

    /// Creates a zero polynomial
    ///
    /// len is the starting length of the data vector. It must be at least 1.
    ///
    /// # Panics
    ///
    /// Panics if len is zero.
    pub fn zero(field: Rc<F>, len: usize) -> Self {
        if len == 0 {
            panic!("Polynomial must have at least one coefficient");
        }
        let data = vec![field.zero(); len];
        Self { field, data }
    }

    /// Creates the monic monomial x<sup>n</sup>
    pub fn x_n(field: Rc<F>, n: usize) -> Self {
        let mut data = vec![field.zero(); n + 1];
        data[n] = field.one();
        Self { field, data }
    }

    /// Creates the polynomial that is the sum of the monic monomials
    /// whose power is given by support
    ///
    /// # Panics
    ///
    /// Panics if support is empty.
    pub fn support(field: Rc<F>, support: &[usize]) -> Self {
        if support.is_empty() {
            panic!("Support cannot be empty");
        }

        let mut data = vec![field.zero(); support[support.len() - 1]];
        for &i in support.iter() {
            if i >= data.len() {
                data.resize(i + 1, field.zero());
            }
            data[i] = field.one();
        }
        Self { field, data }
    }

    pub fn field(&self) -> Rc<F> {
        Rc::clone(&self.field)
    }

    pub fn degree(&self) -> usize {
        let mut degree = self.data.len() - 1;
        while self[degree] == self.field.zero() && degree != 0 {
            degree -= 1;
        }
        degree
    }

    pub fn is_zero(&self) -> bool {
        self.degree() == 0 && self[0] == self.field.zero()
    }

    /// Sets length of vector data as one plus the degree of the polynomial
    fn update_len(&mut self) {
        self.data.truncate(self.degree() + 1);
    }

    /// Returns a random monic polynomial of the chosen degree
    pub fn random(field: Rc<F>, degree: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut p = Self::zero(field, degree + 1);
        for i in 0..degree {
            p[i] = p.field.random_element(&mut rng);
        }
        while p[degree] == p.field.zero() {
            p[degree] = p.field.random_element(&mut rng);
        }
        p
    }

    /// Evaluates polynomial at point
    pub fn eval(&self, point: F::FieldElement) -> F::FieldElement {
        let f = self.field();
        let mut eval = self[0];
        for i in 1..self.degree() + 1 {
            let mut x = f.one();
            for _j in 0..i {
                x = f.mul(x, point);
            }
            eval = f.add(eval, f.mul(self[i], x));
        }
        eval
    }

    /// <https://en.wikipedia.org/wiki/Polynomial_greatest_common_divisor>
    pub fn euclidean_division(a: &Self, b: &Self) -> (Self, Self) {
        if a.field != b.field {
            panic!("Cannot compute euclidean division: fields differ")
        }
        if b.is_zero() {
            panic!("Euclidean division by the null polynomial");
        }
        let f = a.field();
        if a.degree() < b.degree() {
            return (Self::zero(f, 1), a.clone());
        }

        let mut q = Self::zero(Rc::clone(&f), a.degree() - b.degree() + 1);
        let mut r = a.clone();
        let b_deg = b.degree();
        let b_lc_inv = f.inv(b[b_deg]).unwrap();
        while r.degree() >= b_deg && !r.is_zero() {
            let r_deg = r.degree();
            let d = r_deg - b_deg;
            let c = f.mul(r[r_deg], b_lc_inv);
            q[d] = c;
            r[r_deg] = f.zero();
            for i in (0..b_deg).rev() {
                r[i + d] = f.sub(r[i + d], f.mul(c, b[i]));
            }
            r.update_len();
        }
        (q, r)
    }

    pub fn modulo(&mut self, modulus: &Self) {
        let m_deg = modulus.degree();
        let m_lc_inv = self.field.inv(modulus[m_deg]).unwrap();
        while self.degree() >= m_deg && !self.is_zero() {
            let s_deg = self.degree();
            let d = s_deg - m_deg;
            let c = self.field.mul(self[s_deg], m_lc_inv);
            self[s_deg] = self.field.zero();
            for i in (0..m_deg).rev() {
                self[i + d] = self.field.sub(self[i + d], self.field.mul(c, modulus[i]));
            }
        }

        self.update_len();
    }

    pub fn gcd(a: &Self, b: &Self) -> Self {
        if a.field != b.field {
            panic!("Cannot compute euclidean division: fields differ")
        }
        if b.is_zero() {
            return a.clone();
        }

        let (_q, r) = Self::euclidean_division(a, b);
        Self::gcd(b, &r)
    }

    /// <https://en.wikipedia.org/wiki/Polynomial_greatest_common_divisor>
    pub fn extended_gcd(a: &Self, b: &Self) -> (Self, Self, Self, Self, Self) {
        if a.field != b.field {
            panic!("Cannot compute euclidean division: fields differ")
        }

        let f = a.field();
        let mut r = Vec::new();
        let mut s = Vec::new();
        let mut t = Vec::new();
        r.push(a.clone());
        r.push(b.clone());
        let s0 = Self::x_n(Rc::clone(&f), 0);
        let s1 = Self::zero(Rc::clone(&f), 1);
        s.push(s0);
        s.push(s1);
        let t0 = Self::zero(Rc::clone(&f), 1);
        let t1 = Self::x_n(Rc::clone(&f), 0);
        t.push(t0);
        t.push(t1);
        let mut i = 1;
        while !r[i].is_zero() {
            let (quo, rem) = Self::euclidean_division(&r[i - 1], &r[i]);
            r.push(rem);
            s.push(&s[i - 1] - &quo * &s[i]);
            t.push(&t[i - 1] - &quo * &t[i]);
            i += 1;
        }

        r.pop();
        let g = r.pop().unwrap();
        let mut a1 = t.pop().unwrap();
        let v = t.pop().unwrap();
        let mut b1 = s.pop().unwrap();
        let u = s.pop().unwrap();
        if i % 2 == 0 {
            a1 = -a1;
        } else {
            b1 = -b1;
        }

        (g, u, v, a1, b1)
    }

    pub fn inverse_modulo(&self, modulus: &Self) -> Self {
        if self.is_zero() {
            panic!("The null polynom has no inverse");
        }
        let (g, mut inv, _, _, _) = Self::extended_gcd(self, modulus);
        let f = g.field();
        let c = f.inv(g[0]).unwrap();
        for i in 0..inv.degree() + 1 {
            inv[i] = f.mul(c, inv[i]);
        }
        inv
    }
}

pub mod characteristic_two;
pub mod io;
pub mod traits;
