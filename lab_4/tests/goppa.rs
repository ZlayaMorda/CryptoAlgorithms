use log::{info, warn};
use std::rc::Rc;

use mceliece::{finite_field::*, goppa::*, matrix::*, polynomial::*};

pub mod common;

const REPEAT: u32 = 10;

#[test]
fn goppa_f8() {
    common::log_setup();
    let f2 = Rc::new(F2::generate(()));
    let f8 = Rc::new(F2m::generate(8));
    let p = Poly::support(Rc::clone(&f8), &[2, 1, 0]);
    let c = Goppa::new(p, [0, 1, 2, 3, 4, 5, 6, 7].to_vec());
    info!("{}", c);

    let x = c.parity_check_x();
    assert_eq!(x, Mat::new(Rc::clone(&f8), 2, 2, [1, 0, 1, 1].to_vec()));

    let y = c.parity_check_y();
    assert_eq!(
        y,
        Mat::new(
            Rc::clone(&f8),
            2,
            8,
            [1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 2, 3, 4, 5, 6, 7].to_vec()
        )
    );

    let h = c.parity_check_matrix(Rc::clone(&f2));
    info!("Parity-check matrix H:{}", h);

    let (g, _) = Goppa::<F2m>::generator_from_parity_check(&h);
    info!("Generator matrix G:{}", g);

    let z = &h * &g.transpose();
    assert!(z.is_zero());

    let msg = RowVec::random(Rc::clone(&f2), 2);
    info!("Message:{}", msg);

    let cdw = c.encode(&msg);
    info!("Codeword:{}", cdw);

    let err = RowVec::random_with_weight(Rc::clone(&f2), 8, 2);
    info!("Error:{}", err);

    let rcv = &cdw + err;
    info!("Received word:{}", rcv);

    let dcdw = c.decode(&rcv);
    info!("Decoded codeword:{}", dcdw);

    assert_eq!(cdw, dcdw);
}

#[test]
fn goppa_f1024() {
    common::log_setup();
    let f2 = Rc::new(F2::generate(()));
    let f1024 = Rc::new(F2m::generate(1024));
    let c = Goppa::<F2m>::random(f1024, 30, 2);
    info!("{}", c);

    let h = c.parity_check_matrix(Rc::clone(&f2));
    info!("Parity-check matrix H:{}", h);

    let (g, _) = Goppa::<F2m>::generator_from_parity_check(&h);
    assert!((&h * &g.transpose()).is_zero());

    let cdw = RowVec::new(Rc::clone(&f2), g.data()[0..30].to_vec());
    info!("Codeword: {}", cdw);
    let syndrome = Goppa::syndrome_from_xyz(&h, &cdw);

    assert!(syndrome.is_zero());

    let err = RowVec::random_with_weight(Rc::clone(&f2), 30, 2);
    info!("Error:{}", err);

    let rcv = &cdw + &err;
    info!("Received word:{}", rcv);

    let dcdw = c.decode(&rcv);
    info!("Decoded codeword:{}", dcdw);

    assert_eq!(cdw, dcdw);
}

#[test]
fn goppa_f256() {
    common::log_setup();
    let (n, k, t) = (256, 80, 22);
    let f2 = Rc::new(F2::generate(()));
    let f256 = Rc::new(F2m::generate(n));
    let mut g = Poly::support(Rc::clone(&f256), &[22, 17, 15, 12, 5]);
    g[0] = f256.exp(78);
    let l = f256.to_vec();
    let c = Goppa::new(g, l);
    info!("{}", c);

    let msg = RowVec::random(Rc::clone(&f2), k);
    info!("Message:{}", msg);

    let cdw = c.encode(&msg);
    info!("Codeword:{}", cdw);

    let err = RowVec::random_with_weight(Rc::clone(&f2), n, t);
    info!("Error:{}", err);

    let rcv = &cdw + err;
    info!("Received word:{}", rcv);

    let dcdw = c.decode(&rcv);
    info!("Decoded codeword:{}", dcdw);

    assert_eq!(cdw, dcdw);
}

#[test]
fn goppa_f128() {
    common::log_setup();
    let (n, k, t) = (128, 58, 10);
    let f2 = Rc::new(F2::generate(()));
    let fq = Rc::new(F2m::generate(n));
    let goppa = Goppa::<F2m>::random(fq, n, t);
    info!("{}", goppa);

    let msg = RowVec::random(Rc::clone(&f2), k);
    info!("Message:{}", msg);

    let cdw = goppa.encode(&msg);
    info!("Codeword:{}", cdw);

    let err = RowVec::random_with_weight(Rc::clone(&f2), n, t);
    info!("Error:{}", err);

    let rcv = &cdw + err;
    info!("Received word:{}", rcv);

    let dcdw = goppa.decode(&rcv);
    info!("Decoded codeword:{}", dcdw);

    assert_eq!(cdw, dcdw);
}

#[test]
fn goppa_random() {
    let (q, n, t) = common::goppa_setup();
    info!("F2m=F{}, n={}, t={}", q, n, t);

    let f2 = Rc::new(F2::generate(()));
    let f2m = Rc::new(F2m::generate(q));
    let goppa = Goppa::random(f2m, n, t);
    info!("{}", goppa);

    let xyz = goppa.parity_check_xyz();
    info!("XYZ:{}", xyz);

    let h = Goppa::parity_check_from_xyz(&xyz, Rc::clone(&f2));
    info!("Parity-check matrix H:{}", h);

    let (g, _) = Goppa::<F2m>::generator_from_parity_check(&h);
    info!("Generator matrix G:{}", g);

    let k = g.rows();
    info!("Code dimension k = {}", k);

    let msg = RowVec::random(Rc::clone(&f2), k);
    info!("Message:{}", msg);

    let cdw = &msg * &g;
    info!("Codeword:{}", cdw);

    let err = RowVec::random_with_weight(Rc::clone(&f2), n, t);
    info!("Error:{}", err);

    let rcv = &cdw + &err;
    info!("Received word:{}", rcv);

    let dcdw = goppa.xyz_decode(&xyz, &rcv);
    info!("Decoded codeword:{}", dcdw);

    assert_eq!(cdw, dcdw);
}

#[test]
fn goppa_repeat() {
    common::log_setup();
    warn!("Series of {} random encoding - decoding", REPEAT);

    for _r in 0..REPEAT {
        let (q, n, t) = common::goppa_setup();
        warn!(
            "Encode - Decode #{:02}: F2m=F{:<4}, n={:<4}, t={:<4}",
            _r, q, n, t
        );
        goppa_repeated(q, n, t);
    }
}

fn goppa_repeated(q: usize, n: usize, t: usize) {
    let f2 = Rc::new(F2::generate(()));
    let fq = Rc::new(F2m::generate(q));
    let goppa = Goppa::<F2m>::random(fq, n, t);
    info!("{}", goppa);

    let xyz = goppa.parity_check_xyz();
    info!("XYZ:{}", xyz);

    let h = Goppa::parity_check_from_xyz(&xyz, Rc::clone(&f2));
    info!("Parity-check matrix H:{}", h);

    let (g, _) = Goppa::<F2m>::generator_from_parity_check(&h);
    info!("Generator matrix G:{}", g);

    let k = g.rows();
    info!("Code dimension k = {}", k);

    let msg = RowVec::random(Rc::clone(&f2), k);
    info!("Message:{}", msg);

    let cdw = &msg * &g;
    info!("Codeword:{}", cdw);

    let err = RowVec::random_with_weight(Rc::clone(&f2), n, t);
    info!("Error:{}", err);

    let rcv = &cdw + &err;
    info!("Received word:{}", rcv);

    let dcdw = goppa.xyz_decode(&xyz, &rcv);
    info!("Decoded codeword:{}", dcdw);

    assert_eq!(cdw, dcdw);
}
