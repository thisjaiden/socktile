use bevy::prelude::*;
use crate::{components::NewManager, resources::{Netty, Reality}};

pub fn netty_etick(
    mut netty: ResMut<Netty>
) {
    netty.exclusive_tick();
}

pub fn netty_newtick(
    mut netty: ResMut<Netty>,
    manager: Query<&mut NewManager>,
) {
    manager.for_each_mut(|mut man| {
        netty.new_tick(&mut man);
    });
}

pub fn netty_reality(
    mut netty: ResMut<Netty>,
    mut reality: ResMut<Reality>
) {
    netty.reality(&mut reality);
}


mod startup_checks;
pub use startup_checks::startup_checks;
