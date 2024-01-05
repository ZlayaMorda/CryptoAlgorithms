//! The McEliece cryptosystem

use log::debug;
use std::rc::Rc;

use crate::{
    finite_field::{F2m, Field, F2},
    goppa::Goppa,
    matrix::{Mat, Perm, RowVec},
};

/// Public key of the McEliece cryptosystem
///
/// A public key is a couple (SGP, t) where:
/// - SGP is the product of three matrices S, G and P
///   (see struct [`SecretKey`] for more)
/// - t is the correction capacity of the Goppa code (and also the degree of the Goppa polynomial)
///
/// [`SecretKey`]: struct.SecretKey.html
#[derive(Debug, Eq, PartialEq)]
pub struct PublicKey {
    sgp: Mat<F2>,
    t: usize,
}

/// Secret key of the McEliece cryptosystem
///
/// A secret key has three components:
/// - a random invertible matrix S
/// - a random binary irreducible Goppa code of generator matrix G
/// - a random permutation matrix P
///
/// Instead of storing P, we save the corresponding element of the symmetric group.  
/// info_set is the information set of G.  
/// See <https://en.wikipedia.org/wiki/McEliece_cryptosystem>
/// for a brief description of the McEliece cryptosystem.  
/// For more details, see Engelbert, D., Overbeck, R., & Schmidt, A. (2007),
/// A summary of McEliece-type cryptosystems and their security.
/// Journal of Mathematical Cryptology JMC, 1(2), 151-199.
#[derive(Eq, PartialEq)]
pub struct SecretKey {
    s: Mat<F2>,
    goppa: Goppa<F2m>,
    info_set: Vec<usize>,
    p: Perm,
}

pub fn keygen(n: usize, t: usize) -> (PublicKey, SecretKey) {
    let q = if t == 1 && n.is_power_of_two() {
        2 * n
    } else {
        n.next_power_of_two()
    };
    let fq = Rc::new(F2m::generate(q));
    let goppa = Goppa::random(fq, n, t);
    debug!("{}", goppa);

    let xyz = goppa.parity_check_xyz();
    let f2 = Rc::new(F2::generate(()));
    let (g, info_set) = Goppa::generator_from_xyz(&xyz, Rc::clone(&f2));
    debug!("Generator matrix G:{}", g);
    debug!("Information set of generator matrix G:\n{:?}\n", info_set);

    let k = g.rows();
    let s = Mat::invertible_random(f2, k);
    debug!("Code dimension k = {}", k);
    debug!("Singular matrix S:{}", s);

    let p = Perm::random(n);
    debug!("Permutation P:\n{:?}\n", p);

    let sgp = &s * &g * &p;
    debug!("Perturbed generator matrix ~G:{}", sgp);

    let pk = PublicKey { sgp, t };
    let sk = SecretKey {
        s,
        goppa,
        info_set,
        p,
    };
    (pk, sk)
}

impl PublicKey {
    pub fn sgp(&self) -> &Mat<F2> {
        &self.sgp
    }

    pub fn t(&self) -> usize {
        self.t
    }

    pub fn encrypt(&self, m: &RowVec<F2>) -> RowVec<F2> {
        let c = Goppa::<F2m>::g_encode(&self.sgp, m);
        debug!("Encoded plaintext:{}", c);

        let e = RowVec::random_with_weight(m.field(), self.sgp.cols(), self.t);
        debug!("Error vector:{}", e);

        c + e
    }
}

impl SecretKey {
    pub fn s(&self) -> &Mat<F2> {
        &self.s
    }

    pub fn goppa(&self) -> &Goppa<F2m> {
        &self.goppa
    }

    pub fn info_set(&self) -> &Vec<usize> {
        &self.info_set
    }

    pub fn p(&self) -> &Perm {
        &self.p
    }

    pub fn decrypt(&self, c: &RowVec<F2>) -> RowVec<F2> {
        let c1 = c * self.p.inverse();
        let m1 = self.goppa.decode(&c1);
        debug!("Decoded codeword mSG:{}", m1);
        let ms = m1.extract_cols(&self.info_set);
        debug!(
            "Use information set {:?} to extract mS:{}",
            self.info_set, ms
        );

        let m = ms * self.s.inverse().unwrap();
        m
    }
}

pub mod io;
