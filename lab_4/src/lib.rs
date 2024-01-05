pub mod crypto;
pub mod finite_field;
pub mod goppa;
pub mod matrix;
pub mod polynomial;

fn div_ceil(a: usize, b: usize) -> usize {
    a / b + if a % b == 0 { 0 } else { 1 }
}
