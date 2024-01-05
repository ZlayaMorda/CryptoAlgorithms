use log::info;
use rand::Rng;
use std::rc::Rc;

use mceliece::{finite_field::*, matrix::*};

pub mod common;

#[test]
fn matrix_f2_new() {
    common::log_setup();
    let f2 = Rc::new(F2::generate(()));
    let mut a = Mat::zero(Rc::clone(&f2), 3, 4);
    a[(2, 2)] = 1;
    a[(1, 0)] = 1;
    a[(2, 3)] = 1;
    let b = Mat::new(f2, 3, 4, [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1].to_vec());
    assert_eq!(a.rows(), 3);
    assert_eq!(a.cols(), 4);
    assert_eq!(a.data(), b.data());
    assert_eq!(a, b);
}

#[test]
fn matrix_f2_permutation_random() {
    common::log_setup();
    let p = Perm::random(10);
    info!("Permutation matrix:{:?}", p);
    assert!(p.is_permutation());
}

#[test]
fn matrix_f2_is_invertible() {
    common::log_setup();
    let f2 = Rc::new(F2::generate(()));
    let id = Mat::<F2>::identity(f2, 11);
    info!("Matrix identity:{:?}", id);
    assert!(id.is_invertible());
}

#[test]
fn matrix_f2_inverse() {
    common::log_setup();
    let f2 = Rc::new(F2::generate(()));
    let id = Mat::<F2>::identity(f2, 11);
    assert_eq!(id.inverse(), Some(id));

    let p = Perm::random(11);
    assert_eq!(p, p.inverse().inverse());
}

#[test]
fn matrix_f2_invertible_random() {
    common::log_setup();
    let f2 = Rc::new(F2::generate(()));
    let mat = Mat::invertible_random(Rc::clone(&f2), 15);
    assert!(mat.is_invertible());
    assert_eq!(
        mat.inverse()
            .expect("Cannot inverse invertible matrix")
            .inverse()
            .expect("Cannot inverse invertible matrix"),
        mat
    );

    let prod = &mat * &mat.inverse().expect("Cannot inverse invertible matrix");
    let id = Mat::identity(f2, 15);
    assert_eq!(prod, id);
}

#[test]
fn matrix_f2_add() {
    common::log_setup();
    let f2 = Rc::new(F2::generate(()));
    let a = &Mat::random(Rc::clone(&f2), 11, 11);
    let b = &Mat::random(Rc::clone(&f2), 11, 11);
    let c = &Mat::random(Rc::clone(&f2), 11, 11);
    let z = &Mat::zero(Rc::clone(&f2), 11, 11);

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
fn matrix_f2_mul_wrong_dimensions() {
    common::log_setup();
    let f2 = Rc::new(F2::generate(()));
    let a = Mat::zero(Rc::clone(&f2), 5, 4);
    let b = Mat::zero(Rc::clone(&f2), 3, 5);
    let ab = a * b;
    assert!(ab.is_zero());
}

#[test]
fn matrix_f2_mul() {
    common::log_setup();
    let f2 = Rc::new(F2::generate(()));
    let a = &Mat::random(Rc::clone(&f2), 10, 8);
    let b = &Mat::random(Rc::clone(&f2), 8, 13);
    let c = &Mat::random(Rc::clone(&f2), 13, 4);

    // Associativity
    assert_eq!((a * b) * c, a * (b * c));

    // Neutral element
    let i8 = &Mat::identity(Rc::clone(&f2), 8);
    let i10 = &Mat::identity(Rc::clone(&f2), 10);
    assert_eq!(a * i8, *a);
    assert_eq!(i10 * a, *a);

    // Zero case
    let z8 = &Mat::zero(Rc::clone(&f2), 8, 8);
    let z10 = &Mat::zero(Rc::clone(&f2), 10, 10);
    let z10_8 = &Mat::zero(Rc::clone(&f2), 10, 8);
    assert_eq!(a * z8, *z10_8);
    assert_eq!(z10 * a, *z10_8);

    // Distributivity
    let a = &Mat::random(Rc::clone(&f2), 10, 12);
    let b = &Mat::random(Rc::clone(&f2), 10, 12);
    let c = &Mat::random(Rc::clone(&f2), 12, 9);
    let d = &Mat::random(Rc::clone(&f2), 12, 9);
    assert_eq!((a + b) * c, a * c + b * c);
    assert_eq!(a * (c + d), a * c + a * d);
}

#[test]
fn matrix_f2_rank() {
    common::log_setup();
    let f2 = Rc::new(F2::generate(()));
    let mat = Mat::zero(Rc::clone(&f2), 23, 4);
    assert_eq!(mat.rank(), 0);

    let mat = Mat::identity(Rc::clone(&f2), 19);
    assert_eq!(mat.rank(), 19);
}

#[test]
fn matrix_f2_standard_form() {
    common::log_setup();
    let f2 = Rc::new(F2::generate(()));
    let mat = Mat::identity(Rc::clone(&f2), 19);
    assert!(mat.is_standard_form());

    let (u, h, p) = mat
        .standard_form()
        .expect("Cannot recognize the identity matrix as a standard form");
    assert_eq!(u, mat);
    assert_eq!(h, mat);
    assert!(p.is_permutation());

    let mut h = Mat::random(Rc::clone(&f2), 13, 31);
    let inv = Mat::invertible_random(Rc::clone(&f2), 13);
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
fn matrix_f2_random_standard_form() {
    common::log_setup();
    let mut rng = rand::thread_rng();
    let f2 = Rc::new(F2::generate(()));
    let n = rng.gen_range(2, 50);
    let k = rng.gen_range(1, n);
    let h = Mat::random(Rc::clone(&f2), n - k, n);
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
fn matrix_f2_transpose() {
    common::log_setup();
    let mut rng = rand::thread_rng();
    let f2 = Rc::new(F2::generate(()));
    let rows = rng.gen_range(1, 100);
    let cols = rng.gen_range(1, 100);
    let mat = Mat::<F2>::random(f2, rows, cols);
    let tmat = mat.transpose();
    for i in 0..rows {
        for j in 0..cols {
            assert!(mat[(i, j)] == tmat[(j, i)]);
        }
    }
}

#[test]
fn matrix_f2_rowvec_weight() {
    common::log_setup();
    let f2 = Rc::new(F2::generate(()));
    let vec = RowVec::zero(Rc::clone(&f2), 4);
    assert!(vec.weight() == 0);

    let vec = RowVec::random_with_weight(Rc::clone(&f2), 35, 13);
    assert_eq!(vec.weight(), 13);
}

// #[test]
// fn rowvec_f2_write_read() {
//     common::log_setup();
//     let mut rng = rand::thread_rng();
//     let f2 = Rc::new(F2::generate(()));
//     let n = rng.gen_range(10, 1000);
//     let vec = RowVec::random(f2, n);
//     let file_name = "vec_write_read_test.mce";
//     vec.write(file_name).unwrap();
//     let vec_read = RowVec::read_vector(file_name).unwrap();
//     assert_eq!(vec, vec_read);
// }

#[test]
fn matrix_f2_remove_redundant_rows() {
    common::log_setup();
    let f2 = Rc::new(F2::generate(()));
    let v = vec![
        1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut a = Mat::new(Rc::clone(&f2), 5, 5, v);
    let b = Mat::new(Rc::clone(&f2), 2, 5, vec![1, 0, 1, 0, 0, 1, 1, 1, 0, 0]);
    info!("Matrix A:{}", a);

    a.remove_redundant_rows();
    info!("Matrix A without redundant rows:{}", a);

    assert_eq!(a, b);

    let v = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0,
        1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 1, 1, 1, 1,
    ];
    let mut a = Mat::new(Rc::clone(&f2), 13, 5, v);
    let b = Mat::new(
        Rc::clone(&f2),
        3,
        5,
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
    );
    info!("Matrix A:{}", a);

    a.remove_redundant_rows();
    info!("Matrix A without redundant rows:{}", a);

    assert_eq!(a, b);

    let v = vec![
        1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0,
    ];
    let mut a = Mat::new(Rc::clone(&f2), 3, 7, v);
    let b = a.clone();
    info!("Matrix A:{}", a);

    a.remove_redundant_rows();
    info!("Matrix A without redundant rows:{}", a);

    assert_eq!(a, b);
}

#[test]
fn perm_mul_colvec() {
    let f2 = Rc::new(F2::generate(()));
    let p = Perm::new(vec![1, 2, 0]);
    let v = ColVec::new(Rc::clone(&f2), vec![1, 0, 0]);
    let res = ColVec::new(Rc::clone(&f2), vec![0, 1, 0]);
    assert_eq!(p * v, res);
}
