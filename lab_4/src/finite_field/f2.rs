//! The finite field of order 2

use rand::{rngs::ThreadRng, Rng};

use super::{CharacteristicTwo, F2FiniteExtension, Field, FiniteField};

/// Finite field of order 2
#[derive(Eq, PartialEq)]
pub struct F2 {}

impl Field for F2 {
    /// Field Element
    type FieldElement = u32;

    /// Field Parameters
    type FieldParameters = ();

    /// Generates field
    fn generate(_params: Self::FieldParameters) -> Self {
        F2 {}
    }

    /// Returns identity element of field addition
    /// ```
    /// # use mceliece::finite_field::{Field, F2};
    /// let f2 = F2 {};
    /// assert_eq!(f2.zero(), 0);
    /// ```
    fn zero(&self) -> Self::FieldElement {
        0
    }

    /// Returns identity element of field multiplication
    /// ```
    /// # use mceliece::finite_field::{Field, F2};
    /// let f2 = F2 {};
    /// assert_eq!(f2.one(), 1);
    /// ```
    fn one(&self) -> Self::FieldElement {
        1
    }

    /// Returns field characteristic
    /// ```
    /// # use mceliece::finite_field::{Field, F2};
    /// let f2 = F2 {};
    /// assert_eq!(f2.characteristic(), 2);
    /// ```
    fn characteristic(&self) -> usize {
        2
    }

    /// Adds two field elements
    /// ```
    /// # use mceliece::finite_field::{Field, F2};
    /// let f2 = F2 {};
    /// assert_eq!(f2.add(0, 0), 0);
    /// assert_eq!(f2.add(0, 1), 1);
    /// assert_eq!(f2.add(1, 0), 1);
    /// assert_eq!(f2.add(1, 1), 0);
    /// ```
    fn add(&self, a: Self::FieldElement, b: Self::FieldElement) -> Self::FieldElement {
        a ^ b
    }

    /// Adds element b to element a
    /// ```
    /// # use mceliece::finite_field::{Field, F2};
    /// let f2 = F2 {};
    /// let mut a = 1;
    /// f2.add_assign(&mut a, &1);
    /// assert_eq!(a, 0);
    /// ````
    fn add_assign(&self, a: &mut Self::FieldElement, b: &Self::FieldElement) {
        *a = self.add(*a, *b);
    }

    /// Substracts two field elements
    /// ```
    /// # use mceliece::finite_field::{Field, F2};
    /// let f2 = F2 {};
    /// assert_eq!(f2.sub(0, 0), 0);
    /// assert_eq!(f2.sub(0, 1), 1);
    /// assert_eq!(f2.sub(1, 0), 1);
    /// assert_eq!(f2.sub(1, 1), 0);
    /// ```
    fn sub(&self, a: Self::FieldElement, b: Self::FieldElement) -> Self::FieldElement {
        self.add(a, b)
    }

    /// Multiplies two field elements
    /// ```
    /// # use mceliece::finite_field::{Field, F2};
    /// let f2 = F2 {};
    /// assert_eq!(f2.mul(0, 0), 0);
    /// assert_eq!(f2.mul(0, 1), 0);
    /// assert_eq!(f2.mul(1, 0), 0);
    /// assert_eq!(f2.mul(1, 1), 1);
    /// ```
    fn mul(&self, a: Self::FieldElement, b: Self::FieldElement) -> Self::FieldElement {
        a & b
    }

    /// Returns additive inverse of an element
    /// ```
    /// # use mceliece::finite_field::{Field, F2};
    /// let f2 = F2 {};
    /// assert_eq!(f2.neg(0), 0);
    /// assert_eq!(f2.neg(1), 1);
    /// ```
    fn neg(&self, a: Self::FieldElement) -> Self::FieldElement {
        a
    }

    /// Returns multiplicative inverse of an element
    /// ```
    /// # use mceliece::finite_field::{Field, F2};
    /// let f2 = F2 {};
    /// assert_eq!(f2.inv(0), None);
    /// assert_eq!(f2.inv(1), Some(1));
    /// ```
    fn inv(&self, a: Self::FieldElement) -> Option<Self::FieldElement> {
        if a == 0 {
            None
        } else {
            Some(1)
        }
    }

    /// Returns a random element of the field
    fn random_element(&self, rng: &mut ThreadRng) -> Self::FieldElement {
        rng.gen_range(0, 2)
    }
}

impl FiniteField for F2 {
    /// Returns m where field order is p<sup>m</sup> with p prime
    /// ```
    /// # use mceliece::finite_field::{FiniteField, F2};
    /// let f2 = F2 {};
    /// assert_eq!(f2.characteristic_exponent(), 1);
    /// ```
    fn characteristic_exponent(&self) -> u32 {
        1
    }

    /// Returns the nth power of a primitive element
    /// ```
    /// # use mceliece::finite_field::{FiniteField, F2};
    /// let f2 = F2 {};
    /// assert_eq!(f2.exp(0), 1);
    /// assert_eq!(f2.exp(1), 1);
    /// ```
    fn exp(&self, _n: u32) -> Self::FieldElement {
        1
    }

    /// Returns, if it exists, the discrete logarithm of an element
    /// ```
    /// # use mceliece::finite_field::{FiniteField, F2};
    /// let f2 = F2 {};
    /// assert_eq!(f2.log(0), None);
    /// assert_eq!(f2.log(1), Some(0));
    /// ```
    fn log(&self, a: Self::FieldElement) -> Option<u32> {
        if a == 0 {
            None
        } else {
            Some(0)
        }
    }

    /// Represents the element as a string
    fn elt_to_str(&self, a: Self::FieldElement) -> String {
        a.to_string()
    }
}

impl CharacteristicTwo for F2 {}

impl F2FiniteExtension for F2 {
    /// Converts element to an u32
    fn elt_to_u32(&self, a: Self::FieldElement) -> u32 {
        a
    }

    /// Converts an u32 smaller than field order to a field element
    ///
    /// # Panics
    ///
    /// Panics if the u32 is greater than or equal to field order.
    fn u32_to_elt(&self, n: u32) -> Self::FieldElement {
        if n >= self.order() as u32 {
            panic!("u32 must be smaller than field order");
        }
        n
    }
}
