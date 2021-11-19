#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnimatorObject {
    /// id of the animation this object belongs to.
    pub animation_id: usize,
    /// id of the object this object is inside of an animation.
    pub index: usize
}
