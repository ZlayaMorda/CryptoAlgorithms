//! Finite fields of characteristic 2

use rand::{rngs::ThreadRng, Rng};

use super::{CharacteristicTwo, F2FiniteExtension, Field, FiniteField};

/// Finite field of order 2<sup>m</sup>
#[derive(Eq)]
pub struct F2m {
    order: usize,
    m: u32,
    exp: Vec<<Self as Field>::FieldElement>,
    log: Vec<u32>,
}

impl PartialEq for F2m {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

impl Field for F2m {
    /// Field Element
    type FieldElement = u32;

    /// Parameters for field generation
    type FieldParameters = usize;

    /// Generates finite field of given order which is a power of 2
    ///
    /// # Panics
    ///
    /// - Panics if order is not a power of 2.
    /// - Panics if order is 2 (use struct F2 instead).
    /// - Panics if order is greater than 2<sup>16</sup>.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mceliece::finite_field::{Field, FiniteField, F2m};
    /// let f16 = F2m::generate(16);
    /// assert_eq!(f16.order(), 16);
    /// ```
    fn generate(order: Self::FieldParameters) -> Self {
        let (_, m) = match prime_power(order as u32) {
            Ok(r) => r,
            Err(s) => panic!("{}", s),
        };
        let mut f = Self {
            order,
            m,
            exp: vec![0; order],
            log: vec![0; order],
        };
        f.exp[0] = 1;
        f.log[1] = 0;
        let mut elt = 1;
        for i in 1..order {
            elt *= 2;
            if elt >= order as u32 {
                elt ^= primitive_poly(order);
            }
            f.exp[i] = elt;
            f.log[elt as usize] = i as u32;
        }
        f
    }

    /// Returns identity element of field addition
    /// ```
    /// # use mceliece::finite_field::{Field, F2m};
    /// let f2m = F2m::generate(4);
    /// assert_eq!(f2m.zero(), 0);
    /// ```
    fn zero(&self) -> Self::FieldElement {
        0
    }

    /// Returns identity element of field multiplication
    /// ```
    /// # use mceliece::finite_field::{Field, F2m};
    /// let f2m = F2m::generate(8);
    /// assert_eq!(f2m.one(), 1);
    /// ```
    fn one(&self) -> Self::FieldElement {
        1
    }

    /// Returns field characteristic
    /// ```
    /// # use mceliece::finite_field::{Field, F2m};
    /// let f2m = F2m::generate(32);
    /// assert_eq!(f2m.characteristic(), 2);
    /// ```
    fn characteristic(&self) -> usize {
        2
    }

    fn add(&self, a: Self::FieldElement, b: Self::FieldElement) -> Self::FieldElement {
        a ^ b
    }

    /// Adds element b to element a
    /// ```
    /// # use mceliece::finite_field::{Field, F2FiniteExtension, F2m};
    /// let f2m = F2m::generate(32);
    /// let mut a = f2m.u32_to_elt(27);
    /// let b = f2m.u32_to_elt(26);
    /// f2m.add_assign(&mut a, &b);
    /// assert_eq!(a, f2m.one());
    /// ````
    fn add_assign(&self, a: &mut Self::FieldElement, b: &Self::FieldElement) {
        *a = self.add(*a, *b);
    }

    fn sub(&self, a: Self::FieldElement, b: Self::FieldElement) -> Self::FieldElement {
        self.add(a, b)
    }

    /// Multiplies two field elements
    /// ```
    /// # use mceliece::finite_field::{Field, FiniteField, F2m};
    /// let f64 = F2m::generate(64);
    /// let a = f64.exp(4);
    /// let b = f64.exp(11);
    /// let c = f64.exp(59);
    /// assert_eq!(f64.mul(a, b), f64.exp(4 + 11));
    /// assert_eq!(f64.mul(b, c), f64.exp((11 + 59) % 63));
    /// ```
    fn mul(&self, a: Self::FieldElement, b: Self::FieldElement) -> Self::FieldElement {
        let q = self.order as u32;
        let modulo = |a| {
            if a >= q {
                a - (q - 1)
            } else {
                a
            }
        };

        if a == 0 || b == 0 {
            0
        } else {
            self.exp[modulo(self.log[a as usize] + self.log[b as usize]) as usize]
        }
    }

    /// Returns additive inverse of an element
    /// ```
    /// # use mceliece::finite_field::{Field, F2m};
    /// let f256 = F2m::generate(256);
    /// let x = f256.random_element(&mut rand::thread_rng());
    /// assert_eq!(f256.neg(x), x);
    /// ```
    fn neg(&self, a: Self::FieldElement) -> Self::FieldElement {
        a
    }

    fn inv(&self, a: Self::FieldElement) -> Option<Self::FieldElement> {
        let q = self.order as u32;
        if a == 0 {
            None
        } else {
            Some(self.exp[(q - 1 - self.log[a as usize]) as usize])
        }
    }

    fn random_element(&self, rng: &mut ThreadRng) -> Self::FieldElement {
        rng.gen_range(0, self.order as u32)
    }
}

impl FiniteField for F2m {
    /// Returns m where field order is p<sup>m</sup> with p prime
    /// ```
    /// # use mceliece::finite_field::{Field, FiniteField, F2m};
    /// let f128 = F2m::generate(128);
    /// assert_eq!(f128.characteristic_exponent(), 7);
    /// ```
    fn characteristic_exponent(&self) -> u32 {
        self.m
    }

    fn exp(&self, n: u32) -> Self::FieldElement {
        self.exp[n as usize]
    }

    fn log(&self, a: Self::FieldElement) -> Option<u32> {
        if a == 0 {
            None
        } else {
            Some(self.log[a as usize])
        }
    }
}

impl CharacteristicTwo for F2m {}

impl F2FiniteExtension for F2m {
    fn elt_to_u32(&self, a: Self::FieldElement) -> u32 {
        a
    }

    fn u32_to_elt(&self, n: u32) -> Self::FieldElement {
        if n >= self.order() as u32 {
            panic!("u32 must be smaller than field order");
        }
        n
    }
}

/// Returns a primitive polynomial which can be used to generate
/// the finite field of the given order
///
/// The polynomial is returned as a number such that the its binary representation
/// matches the nonzero coefficients of the polynomial.
///
/// # Panics
///
/// - Panics if order is not a power of 2.
/// - Panics if order is 2 (use struct F2 instead).
/// - Panics if order is greater than 2<sup>16</sup>.
pub fn primitive_poly(order: usize) -> u32 {
    match prime_power(order as u32) {
        Ok((_, m)) => match m {
            2 => 0x7,
            3 => 0xB,
            4 => 0x13,
            5 => 0x25,
            6 => 0x43,
            7 => 0x83,
            8 => 0x11D,
            9 => 0x211,
            10 => 0x409,
            11 => 0x805,
            12 => 0x1053,
            13 => 0x201B,
            14 => 0x4143,
            15 => 0x8003,
            16 => 0x110B,
            _ => panic!("m must be at least 2 and at most 16"),
        },
        Err(s) => panic!("{}", s),
    }
}

/// Determines if a number is a prime power.
/// ```
/// # use mceliece::finite_field::f2m::prime_power;
/// assert!(prime_power(2*3).is_err());
/// assert_eq!(prime_power(512), Ok((2, 9)));
/// assert_eq!(prime_power(81), Ok((3, 4)));
/// assert_eq!(prime_power(2_u32.pow(13) - 1), Ok((8191, 1)));
/// ```
pub fn prime_power(q: u32) -> std::result::Result<(u32, u32), &'static str> {
    let mut prime_factors = trial_division(q);
    let m = prime_factors.len() as u32;
    prime_factors.dedup();
    if prime_factors.len() != 1 {
        return Err("Number is not prime");
    }
    let p = prime_factors[0];
    Ok((p, m))
}

/// Computes the prime factors of a nonzero integer by trial division  
/// <https://en.wikipedia.org/wiki/Trial_division>
/// ```
/// # use mceliece::finite_field::f2m::trial_division;
/// assert_eq!(trial_division(1), vec![]);
/// assert_eq!(trial_division(19), vec![19]);
/// assert_eq!(trial_division(77), vec![7, 11]);
/// assert_eq!(trial_division(12), vec![2, 2, 3]);
/// ```
pub fn trial_division(mut n: u32) -> Vec<u32> {
    if n == 0 {
        panic!("0 is an invalid input for trial division");
    }

    let mut prime_factors = Vec::new();
    while n % 2 == 0 {
        prime_factors.push(2);
        n /= 2;
    }
    let mut f = 3;
    while f * f <= n {
        if n % f == 0 {
            prime_factors.push(f);
            n /= f;
        } else {
            f += 2;
        }
    }
    if n != 1 {
        prime_factors.push(n);
    }
    prime_factors
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn f256_add() {
        let f = F2m::generate(256);
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
    fn f256_characteristic() {
        let f = F2m::generate(256);
        let mut rng = rand::thread_rng();
        let a = f.random_element(&mut rng);
        let z = f.zero();

        assert_eq!(f.add(a, a), z);
    }

    #[test]
    fn f256_sub() {
        let f = F2m::generate(256);
        let mut rng = rand::thread_rng();
        let a = f.random_element(&mut rng);
        let b = f.random_element(&mut rng);

        assert_eq!(f.add(a, b), f.sub(a, b));
    }

    #[test]
    fn f256_mul() {
        let f = F2m::generate(256);
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
    fn f256_neg() {
        let f = F2m::generate(256);
        let mut rng = rand::thread_rng();
        let a = f.random_element(&mut rng);
        let b = f.random_element(&mut rng);
        let z = f.zero();

        assert_eq!(f.neg(z), z);
        assert_eq!(f.neg(f.neg(a)), a);
        assert_eq!(f.add(a, f.neg(b)), f.sub(a, b));
    }

    #[test]
    fn f256_inv() {
        let f = F2m::generate(256);
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
