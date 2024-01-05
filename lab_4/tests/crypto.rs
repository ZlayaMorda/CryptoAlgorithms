use log::{info, warn};
use std::rc::Rc;

use mceliece::{crypto::*, finite_field::*, matrix::*};

pub mod common;

const REPEAT: u32 = 10;

#[test]
fn crypto_keygen() {
    let (q, n, t) = common::goppa_setup();
    info!("F2m=F{}, n={}, t={}", q, n, t);

    let (pk, sk) = keygen(n, t);
    let f2 = Rc::new(F2::generate(()));
    let g = sk.goppa().generator_matrix(f2);
    assert!(sk.s().is_invertible());
    assert_eq!(g.rank(), g.rows());
    assert!(sk.p().is_permutation());
    assert_eq!(*pk.sgp(), sk.s() * g * sk.p());
}

#[test]
fn crypto_pk_write_read() {
    let (q, n, t) = common::goppa_setup();
    info!("F2m=F{}, n={}, t={}", q, n, t);

    let (pk, _) = keygen(n, t);
    let file_name = "pk_write_read_test.mce";
    pk.write(file_name).unwrap();
    let pk_read = PublicKey::read_public_key(file_name).unwrap();
    assert_eq!(pk, pk_read);
}

#[test]
fn crypto_sk_write_read() {
    let (q, n, t) = common::goppa_setup();
    info!("F2m=F{}, n={}, t={}", q, n, t);

    let (_, sk) = keygen(n, t);
    let file_name = "sk_write_read_test.mce";
    sk.write(file_name).unwrap();
    let sk_read = SecretKey::read_secret_key(file_name).unwrap();
    assert!(sk == sk_read);
}

#[test]
fn crypto_decrypt_null_ciphertext() {
    let (q, n, t) = common::goppa_setup();
    info!("F2m=F{}, n={}, t={}", q, n, t);

    let f2 = Rc::new(F2::generate(()));
    let (pk, sk) = keygen(n, t);
    let k = pk.sgp().rows();
    let msg = RowVec::zero(Rc::clone(&f2), k);
    let cpt = RowVec::zero(Rc::clone(&f2), n);
    let dmsg = sk.decrypt(&cpt);
    assert_eq!(dmsg, msg);
}

#[test]
fn crypto_decrypt_codeword() {
    let (q, n, t) = common::goppa_setup();
    info!("F2m=F{}, n={}, t={}", q, n, t);

    let f2 = Rc::new(F2::generate(()));
    let (pk, sk) = keygen(n, t);
    let k = pk.sgp().rows();
    let msg = RowVec::random(f2, k);
    let cpt = &msg * pk.sgp();
    let dmsg = sk.decrypt(&cpt);
    assert_eq!(dmsg, msg);
}

#[test]
fn crypto_encrypt_decrypt_null_message() {
    let (q, n, t) = common::goppa_setup();
    info!("F2m=F{}, n={}, t={}", q, n, t);

    let f2 = Rc::new(F2::generate(()));
    let (pk, sk) = keygen(n, t);
    let k = pk.sgp().rows();
    let msg = RowVec::zero(f2, k);
    let cpt = pk.encrypt(&msg);
    assert_eq!(cpt.weight(), t);

    let dmsg = sk.decrypt(&cpt);
    assert_eq!(dmsg, msg);
}

#[test]
fn crypto_encrypt_decrypt_random_message() {
    let (q, n, t) = common::goppa_setup();
    info!("F2m=F{}, n={}, t={}", q, n, t);

    let f2 = Rc::new(F2::generate(()));
    let (pk, sk) = keygen(n, t);
    let k = pk.sgp().rows();
    let msg = RowVec::random(f2, k);
    let cpt = pk.encrypt(&msg);
    let dmsg = sk.decrypt(&cpt);
    assert_eq!(dmsg, msg);
}

#[test]
fn crypto_repeat() {
    common::log_setup();
    warn!("Series of {} random encryption - decryption", REPEAT);

    for _r in 0..REPEAT {
        let (q, n, t) = common::goppa_setup();
        warn!(
            "Encrypt - Decrypt #{:02}: F2m=F{:<4}, n={:<4}, t={:<4}",
            _r, q, n, t
        );
        crypto_repeated(n, t);
    }
}

fn crypto_repeated(n: usize, t: usize) {
    let f2 = Rc::new(F2::generate(()));
    let (pk, sk) = keygen(n, t);
    let k = pk.sgp().rows();
    let msg = RowVec::random(f2, k);
    let cpt = pk.encrypt(&msg);
    let dmsg = sk.decrypt(&cpt);
    assert_eq!(dmsg, msg);
}
