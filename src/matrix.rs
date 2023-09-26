use num::Integer;
use std::any::Any;

pub fn _run_matrix_nxp<R: Iterator<Item = N> + Clone, F: FnMut(N, N), N: Integer + Copy>(
    n: R,
    p: R,
    mut operation: F,
) {
    for x in n {
        for y in p.clone() {
            operation(x, y);
        }
    }
}

/// Runs a function for every position in a matrix.
/// The given matrix is two dimensional and `n` in both width and height.
///
/// # Examples
/// ```
/// use crate::prelude::*;
/// run_matrix_nxn(-1..1, |x, y| {
///     println!("Matrix location ({x}, {y})");
/// });
/// ```
pub fn run_matrix_nxn<R: Iterator<Item = N> + Clone, F: FnMut(N, N), N: Integer + Copy>(
    n: R,
    mut operation: F,
) {
    for x in n.clone() {
        for y in n.clone() {
            operation(x, y);
        }
    }
}

/// Generates a list of coordinate pairs, one for each location in a matrix.
/// The given matrix is two dimensional and `n` in both width and height.
///
/// # Examples
/// ```
/// use crate::prelude::*;
/// assert!(get_matrix_nxn(-1..1).contains((-1, 0)));
/// ```
pub fn get_matrix_nxn<R: Iterator<Item = N> + Clone, N: Integer + Copy>(n: R) -> Vec<(N, N)> {
    let mut out = vec![];
    for x in n.clone() {
        for y in n.clone() {
            out.push((x, y));
        }
    }
    out
}

use rand::seq::SliceRandom;
/// Shortcut to `rand::seq::SliceRandom`'s `Vec<T>.choose()`.
/// 
/// # Panics
/// This function panics if the input has no elements. If this is a concern, use
/// [safe_rand_from_array].
pub fn rand_from_array<T: Any + Clone>(array: Vec<T>) -> T {
    array.choose(&mut rand::thread_rng()).unwrap().clone()
}

/// Shortcut to `rand::seq::SliceRandom`'s `Vec<T>.choose()`. Returns `None` if
/// the input has no elements to choose from.
pub fn safe_rand_from_array<T: Any + Clone>(array: Vec<T>) -> Option<T> {
    let a = array.choose(&mut rand::thread_rng());
    if let Some(a) = a {
        return Some(a.clone());
    }
    else {
        return None;
    }
}

pub fn _all_equal<T: PartialEq>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] == w[1])
}

pub fn _unique_values<T: PartialEq + Copy>(arr: &[T]) -> Vec<T> {
    let mut uniques = vec![];
    for val in arr {
        if !uniques.contains(val) {
            uniques.push(*val);
        }
    }
    return uniques;
}
