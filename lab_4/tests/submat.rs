// use log::info;
// use rand::Rng;
// use std::rc::Rc;

// use mceliece::{finite_field::*, matrix::*};

// pub mod common;

// #[test]
// fn submat() {
//     common::log_setup();
//     let f2 = Rc::new(F2::generate(()));
//     let mut a = Mat::new(
//         f2,
//         4,
//         6,
//         vec![
//             0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1,
//         ],
//     );
//     let a0 = SubMat::new(&a, 1, 2, 2, 4);
//     // a0[(0, 2)] = 0;
//     // a0[(1, 3)] = 1;
//     // let b = Mat::new(
//     //     Field::Some(f2),
//     //     4,
//     //     6,
//     //     vec![
//     //         0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1,
//     //     ],
//     // );
//     // assert_eq!(a, b);
// }
