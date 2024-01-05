//! Finite fields of characteristic 2

use rand::rngs::ThreadRng;

pub use f2::F2;
pub use f2m::F2m;
pub use f7::F7;

/// Field requires implementing Eq as field isomorphism
pub trait Field: Eq {
    /// Field Element
    type FieldElement: Copy + Eq;

    /// Parameters for field generation
    type FieldParameters;

    /// Generates field
    fn generate(params: Self::FieldParameters) -> Self;

    /// Returns identity element of field addition
    fn zero(&self) -> Self::FieldElement;

    /// Returns identity element of field multiplication
    fn one(&self) -> Self::FieldElement;

    /// Returns field characteristic
    fn characteristic(&self) -> usize;

    /// Adds two field elements
    fn add(&self, a: Self::FieldElement, b: Self::FieldElement) -> Self::FieldElement;

    /// Adds element b to element a
    fn add_assign(&self, a: &mut Self::FieldElement, b: &Self::FieldElement);

    /// Substracts two field elements
    fn sub(&self, a: Self::FieldElement, b: Self::FieldElement) -> Self::FieldElement;

    /// Multiplies two field elements
    fn mul(&self, a: Self::FieldElement, b: Self::FieldElement) -> Self::FieldElement;

    /// Returns additive inverse of an element
    fn neg(&self, a: Self::FieldElement) -> Self::FieldElement;

    /// Returns multiplicative inverse of an element
    fn inv(&self, a: Self::FieldElement) -> Option<Self::FieldElement>;

    /// Returns a random element of the field
    fn random_element(&self, rng: &mut ThreadRng) -> Self::FieldElement;
}

pub trait FiniteField: Field {
    /// Returns m where field order is p<sup>m</sup> with p prime
    fn characteristic_exponent(&self) -> u32;

    /// Returns field order
    fn order(&self) -> usize {
        self.characteristic().pow(self.characteristic_exponent())
    }

    /// Returns the nth power of a primitive element
    fn exp(&self, n: u32) -> Self::FieldElement;

    /// Returns, if it exists, the discrete logarithm of an element
    fn log(&self, a: Self::FieldElement) -> Option<u32>;

    /// Converts element to a string as a power of a primitive element
    fn elt_to_str(&self, a: Self::FieldElement) -> String {
        if a == self.zero() {
            "0".to_owned()
        } else if a == self.one() {
            "1".to_owned()
        } else if self.log(a) == Some(1) {
            "a".to_owned()
        } else {
            format!("a^{}", self.log(a).unwrap())
        }
    }
}

pub trait CharacteristicTwo: Field {}

pub trait F2FiniteExtension: CharacteristicTwo + FiniteField {
    /// Converts an element to an u32
    ///
    /// The conversion is such that the binary representation of the u32
    /// matches the decomposition of the element on the canonical basis.
    fn elt_to_u32(&self, a: Self::FieldElement) -> u32;

    /// Converts an u32 to an element
    ///
    /// The conversion is such that the binary representation of the u32
    /// matches the decomposition of the element on the canonical basis.
    ///
    /// # Panics
    ///
    /// Panics if the u32 is greater than or equal to field order.
    fn u32_to_elt(&self, n: u32) -> Self::FieldElement;

    /// Returns a vector containing all elements of the field
    ///
    /// Elements are ranged in ascending order according to their u32 representation
    /// as given by the [`elt_to_u32()`] method.
    ///
    /// [`elt_to_u32()`]: #tymethod.elt_to_u32
    fn to_vec(&self) -> Vec<Self::FieldElement> {
        let q = self.order();
        let mut vec = Vec::with_capacity(q);
        for i in 0..q as u32 {
            vec.push(self.u32_to_elt(i));
        }
        vec
    }
}

pub mod f2;
pub mod f2m;
pub mod f7;
