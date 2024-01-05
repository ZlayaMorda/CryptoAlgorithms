use log::info;
use rand::Rng;
use std::rc::Rc;

use mceliece::{finite_field::*, matrix::*};

pub mod common;

#[test]
fn matrix_f1024_permutation_random() {
    common::log_setup();
    let p = Perm::random(10);
    assert!(p.is_permutation());
}

#[test]
fn matrix_f1024_is_invertible() {
    common::log_setup();
    let f1024 = Rc::new(F2m::generate(1024));
    let id = Mat::identity(Rc::clone(&f1024), 11);
    info!("Matrix identity:{:?}", id);
    assert!(id.is_invertible());
}

#[test]
fn matrix_f1024_inverse() {
    common::log_setup();
    let f1024 = Rc::new(F2m::generate(1024));
    let id = Mat::identity(Rc::clone(&f1024), 11);
    assert_eq!(id.inverse().as_ref(), Some(&id));
}

#[test]
fn matrix_f1024_invertible_random() {
    common::log_setup();
    let f1024 = Rc::new(F2m::generate(1024));
    let mat = Mat::invertible_random(Rc::clone(&f1024), 15);
    assert!(mat.is_invertible());

    assert_eq!(
        mat.inverse()
            .expect("Cannot inverse invertible matrix")
            .inverse()
            .expect("Cannot inverse invertible matrix"),
        mat
    );

    let prod = &mat * &mat.inverse().expect("Cannot inverse invertible matrix");
    let id = Mat::identity(Rc::clone(&f1024), 15);
    assert_eq!(prod, id);
}

#[test]
fn matrix_f1024_add() {
    common::log_setup();
    let f1024 = Rc::new(F2m::generate(1024));
    let a = &Mat::random(Rc::clone(&f1024), 11, 11);
    let b = &Mat::random(Rc::clone(&f1024), 11, 11);
    let c = &Mat::random(Rc::clone(&f1024), 11, 11);
    let z = &Mat::zero(Rc::clone(&f1024), 11, 11);

    // Associativity
    assert_eq!((a + b) + c, a + (b + c));

    // Commutativity
    assert_eq!(a + b, b + a);

    // Neutral element
    assert_eq!(a + z, *a);

    // Characteristic
    assert_eq!(a + a, *z);
}

#[test]
#[should_panic]
fn matrix_f1024_mul_wrong_dimensions() {
    common::log_setup();
    let f1024 = Rc::new(F2m::generate(1024));
    let a = Mat::zero(Rc::clone(&f1024), 5, 4);
    let b = Mat::zero(Rc::clone(&f1024), 3, 5);
    let ab = a * b;
    assert!(ab.is_zero());
}

#[test]
fn matrix_f1024_mul() {
    common::log_setup();
    let f1024 = Rc::new(F2m::generate(1024));
    let a = &Mat::random(Rc::clone(&f1024), 10, 8);
    let b = &Mat::random(Rc::clone(&f1024), 8, 13);
    let c = &Mat::random(Rc::clone(&f1024), 13, 4);

    // Associativity
    assert_eq!((a * b) * c, a * (b * c));

    // Neutral element
    let i8 = &Mat::identity(Rc::clone(&f1024), 8);
    let i10 = &Mat::identity(Rc::clone(&f1024), 10);
    assert_eq!(a * i8, *a);
    assert_eq!(i10 * a, *a);

    // Zero case
    let z8 = &Mat::zero(Rc::clone(&f1024), 8, 8);
    let z10 = &Mat::zero(Rc::clone(&f1024), 10, 10);
    let z10_8 = &Mat::zero(Rc::clone(&f1024), 10, 8);
    assert_eq!(a * z8, *z10_8);
    assert_eq!(z10 * a, *z10_8);

    // Distributivity
    let a = &Mat::random(Rc::clone(&f1024), 10, 12);
    let b = &Mat::random(Rc::clone(&f1024), 10, 12);
    let c = &Mat::random(Rc::clone(&f1024), 12, 9);
    let d = &Mat::random(Rc::clone(&f1024), 12, 9);

    // Left: (a + b)c = ac + bc
    assert_eq!((a + b) * c, a * c + b * c);

    // Right: a(b + c) = ab + ac
    assert_eq!(a * (c + d), a * c + a * d);
}

#[test]
fn matrix_f1024_rank() {
    common::log_setup();
    let f1024 = Rc::new(F2m::generate(1024));
    let mat = Mat::zero(Rc::clone(&f1024), 23, 4);
    assert_eq!(mat.rank(), 0);

    let mat = Mat::identity(Rc::clone(&f1024), 19);
    assert_eq!(mat.rank(), 19);
}

#[test]
fn matrix_f1024_standard_form() {
    common::log_setup();
    let f1024 = Rc::new(F2m::generate(1024));
    let id = Mat::identity(Rc::clone(&f1024), 19);
    assert!(id.is_standard_form());

    let (u, h, p) = id
        .standard_form()
        .expect("Cannot recognize the identity matrix as a standard form");
    assert_eq!(u, id);
    assert_eq!(h, id);
    assert!(p.is_permutation());

    let mut h = Mat::random(Rc::clone(&f1024), 13, 31);
    let inv = Mat::invertible_random(Rc::clone(&f1024), 13);
    for i in 0..13 {
        for j in 0..13 {
            h[(i, j)] = inv[(i, j)];
        }
    }
    let (u, s, p) = h
        .standard_form()
        .expect("Failed to put a full rank matrix in standard form");
    info!("Invertible matrix U:{:?}", u);
    info!("Standard form matrix S:{:?}", s);
    info!("Permutation P:{:?}", p);
    assert!(u.is_invertible());
    assert!(s.is_standard_form());
    assert!(p.is_permutation());
    assert_eq!(s, u * h * p);
}

#[test]
fn matrix_f1024_random_standard_form() {
    common::log_setup();
    let mut rng = rand::thread_rng();
    let f1024 = Rc::new(F2m::generate(1024));
    let n = rng.gen_range(2, 50);
    let k = rng.gen_range(1, n);
    let h = Mat::random(Rc::clone(&f1024), n - k, n);
    if let Some((u, s, p)) = h.random_standard_form() {
        info!("Invertible matrix U:{:?}", u);
        info!("Standard form matrix S:{:?}", s);
        info!("Permutation P:{:?}", p);
        assert!(u.is_invertible());
        assert!(s.is_standard_form());
        assert!(p.is_permutation());
        assert_eq!(s, u * h * p);
    } else {
        info!("Matrix H:{:?}", h);
        assert!(h.rank() < h.rows());
    }
}

#[test]
fn matrix_f1024_transpose() {
    common::log_setup();
    let f1024 = Rc::new(F2m::generate(1024));
    let mut rng = rand::thread_rng();
    let rows = rng.gen_range(1, 100);
    let cols = rng.gen_range(1, 100);
    let mat = Mat::random(Rc::clone(&f1024), rows, cols);
    let tmat = mat.transpose();
    for i in 0..rows {
        for j in 0..cols {
            assert!(mat[(i, j)] == tmat[(j, i)]);
        }
    }
}

#[test]
fn matrix_f1024_rowvec_weight() {
    common::log_setup();
    let f1024 = Rc::new(F2m::generate(1024));
    let vec = RowVec::zero(Rc::clone(&f1024), 4);
    assert!(vec.weight() == 0);

    let vec = RowVec::random_with_weight(Rc::clone(&f1024), 35, 13);
    assert_eq!(vec.weight(), 13);
}
