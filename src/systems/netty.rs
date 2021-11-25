use bevy::prelude::*;
use crate::{components::NewManager, resources::Netty};

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