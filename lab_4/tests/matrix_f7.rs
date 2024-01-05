use log::info;
use rand::Rng;
use std::rc::Rc;

use mceliece::{finite_field::*, matrix::*};

pub mod common;

#[test]
fn matrix_f7_permutation_random() {
    common::log_setup();
    let p = Perm::random(10);
    assert!(p.is_permutation());
}

#[test]
fn matrix_f7_is_invertible() {
    common::log_setup();
    let f7 = Rc::new(F7::generate(()));
    let id = Mat::identity(Rc::clone(&f7), 11);
    info!("Identity matrix:{}", id);
    assert!(id.is_invertible());
}

#[test]
fn matrix_f7_inverse() {
    common::log_setup();
    let f7 = Rc::new(F7::generate(()));
    let id = Mat::identity(Rc::clone(&f7), 11);
    assert!(id.inverse().as_ref() == Some(&id));

    let p = Perm::random(11);
    assert_eq!(p, p.inverse().inverse());
}

#[test]
fn matrix_f7_invertible_random() {
    common::log_setup();
    let f7 = Rc::new(F7::generate(()));
    let mat = Mat::invertible_random(Rc::clone(&f7), 15);
    assert!(mat.is_invertible());
    assert!(
        mat.inverse()
            .expect("Cannot inverse invertible matrix")
            .inverse()
            .expect("Cannot inverse invertible matrix")
            == mat
    );

    let prod = &mat * &mat.inverse().expect("Cannot inverse invertible matrix");
    let id = Mat::identity(Rc::clone(&f7), 15);
    assert!(prod == id);
}

#[test]
fn matrix_f7_add() {
    common::log_setup();
    let f7 = Rc::new(F7::generate(()));
    let a = &Mat::random(Rc::clone(&f7), 11, 11);
    let b = &Mat::random(Rc::clone(&f7), 11, 11);
    let c = &Mat::random(Rc::clone(&f7), 11, 11);
    let z = &Mat::zero(Rc::clone(&f7), 11, 11);

    // Associativity
    assert!((a + b) + c == a + (b + c));

    // Commutativity
    assert!(a + b == b + a);

    // Neutral element
    assert!(a + z == *a);

    // Characteristic
    let mut s = Mat::zero(Rc::clone(&f7), 11, 11);
    for _i in 0..7 {
        s += a;
    }
    assert!(s == *z);
}

#[test]
#[should_panic]
fn matrix_f7_mul_wrong_dimensions() {
    common::log_setup();
    let f7 = Rc::new(F7::generate(()));
    let a = Mat::zero(Rc::clone(&f7), 5, 4);
    let b = Mat::zero(Rc::clone(&f7), 3, 5);
    let ab = a * b;
    assert!(ab.is_zero());
}

#[test]
fn matrix_f7_mul() {
    common::log_setup();
    let f7 = Rc::new(F7::generate(()));
    let a = &Mat::random(Rc::clone(&f7), 10, 8);
    let b = &Mat::random(Rc::clone(&f7), 8, 13);
    let c = &Mat::random(Rc::clone(&f7), 13, 4);

    // Associativity
    assert!((a * b) * c == a * (b * c));

    // Neutral element
    let i8 = &Mat::identity(Rc::clone(&f7), 8);
    let i10 = &Mat::identity(Rc::clone(&f7), 10);
    assert!(a * i8 == *a);
    assert!(i10 * a == *a);

    // Zero case
    let z8 = &Mat::zero(Rc::clone(&f7), 8, 8);
    let z10 = &Mat::zero(Rc::clone(&f7), 10, 10);
    let z10_8 = &Mat::zero(Rc::clone(&f7), 10, 8);
    assert!(a * z8 == *z10_8);
    assert!(z10 * a == *z10_8);

    // Distributivity
    let a = &Mat::random(Rc::clone(&f7), 10, 12);
    let b = &Mat::random(Rc::clone(&f7), 10, 12);
    let c = &Mat::random(Rc::clone(&f7), 12, 9);
    let d = &Mat::random(Rc::clone(&f7), 12, 9);

    // Left: (a + b)c = ac + bc
    assert!((a + b) * c == a * c + b * c);

    // Right: a(b + c) = ab + ac
    assert!(a * (c + d) == a * c + a * d);
}

#[test]
fn matrix_f7_rank() {
    common::log_setup();
    let f7 = Rc::new(F7::generate(()));
    let mat = Mat::zero(Rc::clone(&f7), 23, 4);
    assert!(mat.rank() == 0);

    let mat = Mat::identity(Rc::clone(&f7), 19);
    assert!(mat.rank() == 19);
}

#[test]
fn matrix_f7_standard_form() {
    common::log_setup();
    let f7 = Rc::new(F7::generate(()));
    let id = Mat::identity(Rc::clone(&f7), 19);
    assert!(id.is_standard_form());

    let (u, h, p) = id
        .standard_form()
        .expect("Cannot recognize the identity matrix as a standard form");
    assert!(u == id);
    assert!(h == id);
    assert!(p.is_permutation());

    let mut h = Mat::random(Rc::clone(&f7), 13, 31);
    let inv = Mat::invertible_random(Rc::clone(&f7), 13);
    for i in 0..13 {
        for j in 0..13 {
            h[(i, j)] = inv[(i, j)];
        }
    }
    let (u, s, p) = h
        .standard_form()
        .expect("Failed to put a full rank matrix in standard form");
    info!("Invertible matrix U:{}", u);
    info!("Standard form matrix S:{}", s);
    info!("Permutation P:{:?}", p);
    assert!(u.is_invertible());
    assert!(s.is_standard_form());
    assert!(p.is_permutation());
    assert!(s == u * h * p);
}

#[test]
fn matrix_f7_random_standard_form() {
    common::log_setup();
    let mut rng = rand::thread_rng();
    let f7 = Rc::new(F7::generate(()));
    let n = rng.gen_range(2, 50);
    let k = rng.gen_range(1, n);
    let h = Mat::random(Rc::clone(&f7), n - k, n);
    if let Some((u, s, p)) = h.random_standard_form() {
        info!("Invertible matrix U:{}", u);
        info!("Standard form matrix S:{}", s);
        info!("Permutation P:{:?}", p);
        assert!(u.is_invertible());
        assert!(s.is_standard_form());
        assert!(p.is_permutation());
        assert!(s == u * h * p);
    } else {
        info!("Matrix H:{}", h);
        info!("rank of H: {}\n", h.rank());
        assert!(h.rank() < h.rows());
    }
}

#[test]
fn matrix_f7_random_standard_form_0() {
    common::log_setup();
    let f7 = Rc::new(F7::generate(()));
    let h = Mat::new(f7, 2, 3, vec![3, 1, 6, 3, 1, 5]);
    if let Some((u, s, p)) = h.random_standard_form() {
        info!("Invertible matrix U:{}", u);
        info!("Standard form matrix S:{}", s);
        info!("Permutation P:{:?}", p);
        assert!(u.is_invertible());
        assert!(s.is_standard_form());
        assert!(p.is_permutation());
        assert!(s == u * h * p);
    } else {
        info!("Matrix H:{}", h);
        panic!();
    }
}

#[test]
fn matrix_f7_transpose() {
    common::log_setup();
    let f7 = Rc::new(F7::generate(()));
    let mut rng = rand::thread_rng();
    let rows = rng.gen_range(1, 100);
    let cols = rng.gen_range(1, 100);
    let mat = Mat::random(Rc::clone(&f7), rows, cols);
    let tmat = mat.transpose();
    for i in 0..rows {
        for j in 0..cols {
            assert!(mat[(i, j)] == tmat[(j, i)]);
        }
    }
}

#[test]
fn matrix_f7_rowvec_weight() {
    common::log_setup();
    let f7 = Rc::new(F7::generate(()));
    let vec = RowVec::zero(Rc::clone(&f7), 4);
    assert!(vec.weight() == 0);

    let vec = RowVec::random_with_weight(Rc::clone(&f7), 35, 13);
    assert!(vec.weight() == 13);
}
