use crate::shared::world::World;

pub struct PlayManager {
    pub world: World
}

impl PlayManager {
    pub fn new() -> PlayManager {
        PlayManager {
            world: World::empty()
        }
    }
}
