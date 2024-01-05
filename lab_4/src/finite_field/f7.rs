//! The finite field of order 7

use rand::{rngs::ThreadRng, Rng};

use super::{Field, FiniteField};

/// Field order
const ORDER: usize = 7;

const EXP: [F7Element; ORDER] = [1, 3, 2, 6, 4, 5, 1];

const LOG: [u32; ORDER] = [ORDER as u32, 0, 2, 1, 4, 5, 3];

/// F7 Element
type F7Element = u32;

/// Finite field of order 7
#[derive(Eq)]
pub struct F7 {}

impl PartialEq for F7 {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Field for F7 {
    /// Field Element
    type FieldElement = F7Element;

    /// Parameters for field generation
    type FieldParameters = ();

    /// Generates field
    fn generate(_params: Self::FieldParameters) -> Self {
        F7 {}
    }

    /// Returns identity element of field addition
    /// ```
    /// # use mceliece::finite_field::{Field, F7};
    /// let f7 = F7 {};
    /// assert_eq!(f7.zero(), 0);
    /// ```
    fn zero(&self) -> Self::FieldElement {
        0
    }

    /// Returns identity element of field multiplication
    /// ```
    /// # use mceliece::finite_field::{Field, F7};
    /// let f7 = F7 {};
    /// assert_eq!(f7.one(), 1);
    /// ```
    fn one(&self) -> Self::FieldElement {
        1
    }

    /// Returns field characteristic
    /// ```
    /// # use mceliece::finite_field::{Field, F7};
    /// let f7 = F7 {};
    /// assert_eq!(f7.characteristic(), 7);
    /// ```
    fn characteristic(&self) -> usize {
        ORDER
    }

    /// Adds two field elements
    /// ```
    /// # use mceliece::finite_field::{Field, F7};
    /// let f7 = F7 {};
    /// assert_eq!(f7.add(0, 3), 3);
    /// assert_eq!(f7.add(5, 1), 6);
    /// assert_eq!(f7.add(6, 1), 0);
    /// assert_eq!(f7.add(6, 3), 2);
    /// ```
    fn add(&self, a: Self::FieldElement, b: Self::FieldElement) -> Self::FieldElement {
        (a + b) % (ORDER as u32)
    }

    /// Adds element b to element a
    /// ```
    /// # use mceliece::finite_field::{Field, F7};
    /// let f7 = F7 {};
    /// let mut a = 5;
    /// let b = 4;
    /// f7.add_assign(&mut a, &b);
    /// assert_eq!(a, 2);
    /// ````
    fn add_assign(&self, a: &mut Self::FieldElement, b: &Self::FieldElement) {
        *a = self.add(*a, *b);
    }

    /// Substracts two field elements
    /// ```
    /// # use mceliece::finite_field::{Field, F7};
    /// let f7 = F7 {};
    /// assert_eq!(f7.sub(6, 4), 2);
    /// assert_eq!(f7.sub(5, 0), 5);
    /// assert_eq!(f7.sub(0, 1), 6);
    /// assert_eq!(f7.sub(2, 4), 5);
    /// ```
    fn sub(&self, a: Self::FieldElement, b: Self::FieldElement) -> Self::FieldElement {
        let q = ORDER as u32;
        (q + a - b) % q
    }

    fn mul(&self, a: Self::FieldElement, b: Self::FieldElement) -> Self::FieldElement {
        let q = ORDER as u32;
        let modulo = |x| {
            if x >= q {
                x - (q - 1)
            } else {
                x
            }
        };

        if a == 0 || b == 0 {
            0
        } else {
            EXP[modulo(LOG[a as usize] + LOG[b as usize]) as usize]
        }
    }

    /// Returns additive inverse of an element
    /// ```
    /// # use mceliece::finite_field::{Field, F7};
    /// let f7 = F7 {};
    /// assert_eq!(f7.neg(0), 0);
    /// assert_eq!(f7.neg(1), 6);
    /// assert_eq!(f7.neg(3), 4);
    /// assert_eq!(f7.neg(4), 3);
    /// ```
    fn neg(&self, a: Self::FieldElement) -> Self::FieldElement {
        let q = ORDER as u32;
        (q - a) % q
    }

    fn inv(&self, a: Self::FieldElement) -> Option<Self::FieldElement> {
        let q = ORDER as u32;
        if a == 0 {
            None
        } else {
            Some(EXP[(q - 1 - LOG[a as usize]) as usize])
        }
    }

    fn random_element(&self, rng: &mut ThreadRng) -> Self::FieldElement {
        rng.gen_range(0, ORDER as u32)
    }
}

impl FiniteField for F7 {
    /// Returns m where field order is p<sup>m</sup> with p prime
    /// ```
    /// # use mceliece::finite_field::{FiniteField, F7};
    /// let f7 = F7 {};
    /// assert_eq!(f7.characteristic_exponent(), 1);
    /// ```
    fn characteristic_exponent(&self) -> u32 {
        1
    }

    fn exp(&self, n: u32) -> Self::FieldElement {
        EXP[n as usize]
    }

    fn log(&self, a: Self::FieldElement) -> Option<u32> {
        if a == 0 {
            None
        } else {
            Some(LOG[a as usize])
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn f7_add() {
        let f = &F7 {};
        let mut rng = rand::thread_rng();
        let a = f.random_element(&mut rng);
        let b = f.random_element(&mut rng);
        let c = f.random_element(&mut rng);
        let z = f.zero();

        assert_eq!(f.add(a, f.add(b, c)), f.add(f.add(a, b), c));
        assert_eq!(f.add(a, b), f.add(b, a));
        assert_eq!(f.add(a, z), a);
    }

    #[test]
    fn f7_characteristic() {
        let f = &F7 {};
        let mut rng = rand::thread_rng();
        let a = f.random_element(&mut rng);
        let z = f.zero();
        let mut s = f.zero();
        for _i in 0..7 {
            s = f.add(s, a);
        }

        assert_eq!(s, z);
    }

    #[test]
    fn f7_sub() {
        let f = &F7 {};
        let mut rng = rand::thread_rng();
        let a = f.random_element(&mut rng);
        let z = f.zero();

        assert_eq!(f.sub(a, z), a);
        assert_eq!(f.sub(a, a), z);
    }

    #[test]
    fn f7_mul() {
        let f = &F7 {};
        let mut rng = rand::thread_rng();
        let a = f.random_element(&mut rng);
        let b = f.random_element(&mut rng);
        let c = f.random_element(&mut rng);
        let i = f.one();
        let z = f.zero();

        assert_eq!(f.mul(a, f.mul(b, c)), f.mul(f.mul(a, b), c));
        assert_eq!(f.mul(a, b), f.mul(b, a));
        assert_eq!(f.mul(a, i), a);
        assert_eq!(f.mul(a, z), z);
        assert_eq!(f.mul(a, f.add(b, c)), f.add(f.mul(a, b), f.mul(a, c)));
        assert_eq!(f.mul(a, f.sub(b, c)), f.sub(f.mul(a, b), f.mul(a, c)));
    }

    #[test]
    fn f7_neg() {
        let f = &F7 {};
        let mut rng = rand::thread_rng();
        let a = f.random_element(&mut rng);
        let b = f.random_element(&mut rng);
        let z = f.zero();

        assert_eq!(f.neg(z), z);
        assert_eq!(f.neg(f.neg(a)), a);
        assert_eq!(f.add(a, f.neg(b)), f.sub(a, b));
    }

    #[test]
    fn f7_inv() {
        let f = &F7 {};
        let mut rng = rand::thread_rng();
        let a = f.random_element(&mut rng);
        let i = f.one();
        let z = f.zero();

        assert_eq!(f.inv(z), None);
        assert_eq!(f.inv(i), Some(i));
        if a != f.zero() {
            assert_eq!(f.inv(f.inv(a).unwrap()), Some(a));
            assert_eq!(f.mul(a, f.inv(a).unwrap()), i);
        }
    }
}
