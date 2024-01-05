use std::rc::Rc;

use super::Poly;
use crate::finite_field::{f2m, CharacteristicTwo, F2FiniteExtension, Field};

impl<F> Poly<F>
where
    F: CharacteristicTwo + Field,
{
    pub fn random_monic_irreducible(field: Rc<F>, degree: usize) -> Self
    where
        F: F2FiniteExtension,
    {
        let mut rng = rand::thread_rng();
        let mut p = Self::zero(field, degree + 1);
        p[degree] = p.field.one();
        for i in 0..degree {
            p[i] = p.field.random_element(&mut rng);
        }
        while !p.is_irreducible() {
            for i in 0..degree {
                p[i] = p.field.random_element(&mut rng);
            }
        }
        p
    }

    pub fn is_irreducible(&self) -> bool
    where
        F: F2FiniteExtension,
    {
        let n = self.degree() as u32;
        if n == 0 {
            return false;
        }
        if n == 1 {
            return true;
        }

        let f = self.field();
        let q = f.order() as u32;
        let mut n_prime_factors = f2m::trial_division(n);
        n_prime_factors.dedup();
        let n_div_primes: Vec<u32> = n_prime_factors.iter().map(|x| n / x).collect();

        for i in 0..n_prime_factors.len() {
            let mut h = Self::x_n(Rc::clone(&f), 1);
            for _j in 0..n_div_primes[i] {
                h.pow_modulo(q, self);
            }
            h -= &Self::x_n(Rc::clone(&f), 1);
            let g = Self::gcd(self, &h);
            if g.degree() != 0 {
                return false;
            }
        }
        let mut g = Self::x_n(Rc::clone(&f), 1);
        for _i in 0..n {
            g.pow_modulo(q, self);
        }
        g -= &Self::x_n(Rc::clone(&f), 1);
        g.modulo(self);
        g.is_zero()
    }

    pub fn square_root_modulo(&mut self, modulus: &Self)
    where
        F: F2FiniteExtension,
    {
        if self.field != modulus.field {
            panic!("Cannot compute square root modulo: fields don't match");
        }
        let m = self.field.characteristic_exponent() as usize;

        for _i in 0..m * modulus.degree() - 1 {
            self.square();
            self.modulo(modulus);
        }
    }

    pub fn inverse_modulo_by_fast_exponentiation(&mut self, modulus: &Self)
    where
        F: F2FiniteExtension,
    {
        if self.field != modulus.field {
            panic!("Cannot compute inverse modulo: fields don't match");
        }
        if self.is_zero() {
            panic!("The null polynom has no inverse");
        }
        let m = self.field.characteristic_exponent() as usize;

        self.square();
        let mut tmp = self.clone();
        for _i in 0..m * modulus.degree() - 2 {
            tmp.square();
            tmp.modulo(&modulus);
            *self *= &tmp;
            self.modulo(&modulus);
        }
    }

    pub fn square(&mut self) {
        let t = self.degree();
        self.data.resize(2 * t + 1, self.field.zero());

        for i in (1..t + 1).rev() {
            self[2 * i] = self.field.mul(self[i], self[i]);
            self[2 * i - 1] = self.field.zero();
        }
        self[0] = self.field.mul(self[0], self[0]);
    }

    /// Computes p<sup>n</sup> mod (modulus)
    pub fn pow_modulo(&mut self, mut n: u32, modulus: &Self) {
        if self.field != modulus.field {
            panic!("Cannot compute power modulo: fields don't match");
        }
        self.modulo(modulus);
        let mut tmp = self.clone();
        self.data.truncate(1);
        self[0] = self.field.one();
        if n & 1 == 1 {
            *self *= &tmp;
        }
        n >>= 1;
        while n != 0 {
            tmp.square();
            tmp.modulo(modulus);
            if n & 1 == 1 {
                *self *= &tmp;
                self.modulo(modulus);
            }
            n >>= 1;
        }
    }

    pub(crate) fn goppa_extended_gcd(g: &Self, t: &Self) -> (Self, Self) {
        if g.field != t.field {
            panic!("Cannot compute euclidean division: fields differ")
        }

        let f = g.field();
        let mut i = 1;

        let mut a = Vec::new();
        let mut b = Vec::new();
        a.push(g.clone());
        a.push(t.clone());
        let b0 = Self::zero(Rc::clone(&f), 1);
        let b1 = Self::x_n(Rc::clone(&f), 0);
        b.push(b0);
        b.push(b1);

        while a[i].degree() > g.degree() / 2 {
            i += 1;
            let (q, r) = Self::euclidean_division(&a[i - 2], &a[i - 1]);
            b.push(&b[i - 2] - &q * &b[i - 1]);
            a.push(r);
        }

        (a.pop().unwrap(), b.pop().unwrap())
    }
}
