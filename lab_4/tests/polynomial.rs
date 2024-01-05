use log::info;
use rand::Rng;
use std::rc::Rc;

use mceliece::{finite_field::*, polynomial::*};

pub mod common;

#[test]
fn polynomial_f2_division() {
    common::log_setup();
    let f = Rc::new(F2::generate(()));
    let a = Poly::support(Rc::clone(&f), &[0, 2, 4]);
    info!("a(x) = {:?}\n", a);

    let b = Poly::support(Rc::clone(&f), &[0, 1, 2]);
    info!("b(x) = {:?}\n", b);

    let (q, r) = Poly::euclidean_division(&a, &b);
    info!("q(x) = {:?}\n", q);
    info!("r(x) = {:?}\n", r);

    assert_eq!(b, q);
    assert!(r.is_zero());
}

#[test]
fn polynomial_f2_gcd() {
    common::log_setup();
    let f = Rc::new(F2::generate(()));
    let a = Poly::support(Rc::clone(&f), &[0, 1, 4]);
    info!("a(x) = {:?}\n", a);

    let b = Poly::support(Rc::clone(&f), &[0, 1, 2]);
    info!("b(x) = {:?}\n", b);

    let c = Poly::support(Rc::clone(&f), &[0, 1]);
    info!("c(x) = {:?}\n", c);

    let ac = &a * &c;
    info!("a(x)c(x) = {:?}\n", ac);

    let bc = &b * &c;
    info!("b(x)c(x) = {:?}\n", bc);

    let d = Poly::gcd(&ac, &bc);
    info!("d(x) = gcd(a(x)c(x), b(x)c(x)) = {:?}\n", d);

    assert_eq!(c, d);
}

#[test]
fn polynomial_f2_extended_gcd() {
    common::log_setup();
    let f = Rc::new(F2::generate(()));
    let mut rng = rand::thread_rng();
    let deg_a = rng.gen_range(1, 100);
    let a = Poly::random(Rc::clone(&f), deg_a);
    info!("a(x) = {:?}\n", a);

    let deg_b = rng.gen_range(0, deg_a);
    let b = Poly::random(Rc::clone(&f), deg_b);
    info!("b(x) = {:?}\n", b);

    let (d, u, v, a1, b1) = Poly::extended_gcd(&a, &b);
    info!("d(x) = {:?}", d);
    info!("u(x) = {:?}", u);
    info!("v(x) = {:?}", v);
    info!("a1(x) = {:?}", a1);
    info!("b1(x) = {:?}", b1);

    assert_eq!(a, &d * &a1);
    assert_eq!(b, &d * &b1);
    assert_eq!(d, &a * &u + &b * &v);
}

#[test]
fn polynomial_f7_extended_gcd() {
    common::log_setup();
    let f = Rc::new(F7::generate(()));
    let mut rng = rand::thread_rng();
    let deg_a = rng.gen_range(1, 100);
    let a = Poly::random(Rc::clone(&f), deg_a);
    info!("a(x) = {}\n", a);

    let deg_b = rng.gen_range(0, deg_a);
    let b = Poly::random(Rc::clone(&f), deg_b);
    info!("b(x) = {}\n", b);

    let (d, u, v, a1, b1) = Poly::extended_gcd(&a, &b);
    info!("d(x) = {}", d);
    info!("u(x) = {}", u);
    info!("v(x) = {}", v);
    info!("a1(x) = {}", a1);
    info!("b1(x) = {}", b1);

    assert!(a == &d * &a1);
    assert!(b == &d * &b1);
    assert!(d == &a * &u + &b * &v);
}

#[test]
fn polynomial_f1024_extended_gcd() {
    common::log_setup();
    let f = Rc::new(F2m::generate(1024));
    let mut rng = rand::thread_rng();
    let deg_a = rng.gen_range(1, 100);
    let a = Poly::random(Rc::clone(&f), deg_a);
    info!("a(x) = {:?}\n", a);

    let deg_b = rng.gen_range(0, deg_a);
    let b = Poly::random(Rc::clone(&f), deg_b);
    info!("b(x) = {:?}\n", b);

    let (d, u, v, a1, b1) = Poly::extended_gcd(&a, &b);
    info!("d(x) = {:?}", d);
    info!("u(x) = {:?}", u);
    info!("v(x) = {:?}", v);
    info!("a1(x) = {:?}", a1);
    info!("b1(x) = {:?}", b1);

    assert_eq!(a, &d * &a1);
    assert_eq!(b, &d * &b1);
    assert_eq!(d, &a * &u + &b * &v);
}

#[test]
fn polynomial_f1024_square() {
    common::log_setup();
    let mut rng = rand::thread_rng();
    let deg_a = rng.gen_range(0, 100);
    let f1024 = Rc::new(F2m::generate(1024));
    let mut a = Poly::<F2m>::random(f1024, deg_a);
    info!("a(x) = {:?}\n", a);

    let b = a.clone();
    a.square();
    assert_eq!(a, &b * &b);
}

#[test]
fn polynomial_f1024_modulo() {
    common::log_setup();
    let f = Rc::new(F2m::generate(1024));
    let mut rng = rand::thread_rng();
    let deg_a = rng.gen_range(0, 100);
    let mut a = Poly::random(Rc::clone(&f), deg_a);
    info!("a(x) = {:?}\n", a);

    let deg_g = rng.gen_range(0, 100);
    let g = Poly::random(Rc::clone(&f), deg_g);
    info!("g(x) = {:?}\n", g);

    let (_q, r) = Poly::euclidean_division(&a, &g);
    a.modulo(&g);
    assert_eq!(a, r);
}

#[test]
fn polynomial_f1024_sq_root_mod() {
    common::log_setup();
    let f = Rc::new(F2m::generate(1024));
    let mut rng = rand::thread_rng();
    let deg_a = rng.gen_range(0, 11);
    let a = Poly::random(Rc::clone(&f), deg_a);
    info!("a(x) = {:?}\n", a);

    let mut b = &a * &a;
    info!("a^2(x) = {:?}\n", b);

    let g = Poly::support(Rc::clone(&f), &[0, 2, 11]);
    b.square_root_modulo(&g);
    info!("b(x) = {:?}\n", b);

    assert_eq!(a, b);
}

#[test]
fn polynomial_f1024_inverse_mod() {
    common::log_setup();
    let f = Rc::new(F2m::generate(1024));
    let g = Poly::support(Rc::clone(&f), &[0, 2, 11]);
    let mut rng = rand::thread_rng();
    let deg_a = rng.gen_range(0, 11);
    let a = Poly::random(Rc::clone(&f), deg_a);
    info!("a(x) = {:?}\n", a);

    let inv = a.inverse_modulo(&g);
    info!("a^-1(x) = {:?}\n", inv);

    let mut p = &a * &inv;
    p.modulo(&g);
    let id = Poly::x_n(Rc::clone(&f), 0);
    assert_eq!(p, id);
}

#[test]
fn polynomial_is_irreducible() {
    common::log_setup();
    let f2 = Rc::new(F2::generate(()));
    let f1024 = Rc::new(F2m::generate(1024));

    let zero = Poly::zero(Rc::clone(&f2), 1);
    assert!(!zero.is_irreducible());

    let mut constant_poly = Poly::zero(Rc::clone(&f1024), 1);
    constant_poly[0] = f1024.exp(533);
    assert!(!constant_poly.is_irreducible());

    let p = Poly::support(Rc::clone(&f2), &[0, 1, 2]);
    assert!(p.is_irreducible());

    let p = Poly::support(Rc::clone(&f2), &[0, 2]);
    assert!(!p.is_irreducible());

    let p = Poly::support(Rc::clone(&f1024), &[0, 2, 11]);
    assert!(p.is_irreducible());

    let p = Poly::support(Rc::clone(&f2), &[0, 3, 10]);
    assert!(p.is_irreducible());

    let p = Poly::support(Rc::clone(&f1024), &[0, 3, 10]);
    assert!(!p.is_irreducible());
}
