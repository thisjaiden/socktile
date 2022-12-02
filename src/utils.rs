use crate::prelude::*;

/// Calculates the distance between two `Transform`s. Does not take into account the Z-axis.
pub fn distance(a: Transform, b: Transform) -> f32 {
    // d=√((x_2-x_1)²+(y_2-y_1)²)
    (((b.translation.x - a.translation.x).powi(2))+((b.translation.y - a.translation.y).powi(2))).sqrt()
}
