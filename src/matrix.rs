use std::any::Any;
use num::Integer;
use rand::seq::SliceRandom;

pub fn _run_matrix_nxp<R: Iterator<Item = N> + Clone, F: FnMut(N, N), N: Integer + Copy>(n: R, p: R, mut operation: F) {
    for x in n {
        for y in p.clone() {
            operation(x, y);
        }
    }
}

/// Runs a function for every position in a matrix.
/// The given matrix two dimensional and N by N in size.
/// 
/// # Examples
/// ```
/// use crate::prelude::*;
/// run_matrix_nxn(-1..1, |x, y| {
///     println!("Matrix location ({x}, {y})");
/// });
/// ```
pub fn run_matrix_nxn<R: Iterator<Item = N> + Clone, F: FnMut(N, N), N: Integer + Copy>(n: R, mut operation: F) {
    for x in n.clone() {
        for y in n.clone() {
            operation(x, y);
        }
    }
}

pub fn get_matrix_nxn<R: Iterator<Item = N> + Clone, N: Integer + Copy>(n: R) -> Vec<(N, N)> {
    let mut out = vec![];
    for x in n.clone() {
        for y in n.clone() {
            out.push((x, y));
        }
    }
    out
}

pub fn rand_from_array<T: Any + Clone>(array: Vec<T>) -> T {
    array.choose(&mut rand::thread_rng()).unwrap().clone()
}
