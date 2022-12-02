use crate::prelude::*;
use std::net::SocketAddr;
use super::{SaveGame, Profile};

use std::sync::{Arc, Mutex};

#[allow(non_snake_case)]
pub fn tick(globals: Arc<Mutex<Globals>>) -> Vec<(Packet, SocketAddr)> {
    let mut outgoing: Vec<(Packet, SocketAddr)> = vec![];
    let mut glob_access = globals.lock().unwrap();
    if glob_access.last_autosave.elapsed() > AUTOSAVE_FREQUENCY {
        glob_access.last_autosave = std::time::Instant::now();
        info!("Saving worlds and profiles");
        for world in glob_access.worlds.clone() {
            save_world(world);
        }
        for profile in glob_access.profiles.clone() {
            save_profile(profile);
        }
        info!("Done saving");
    }
    // For every world...
    let ips = glob_access.user_to_addr.clone();
    for server in &mut glob_access.worlds {
        let mut removed = 0;
        // For every object...
        'object: for (object_index, object) in server.data.objects.clone().iter().enumerate() {
            // If the object is an item...
            if let ObjectType::GroundItem(item) = object.rep {
                let server_players = &server.data.players;
                // Item pickup
                // For every player...
                for (index, (_user, pos, data)) in server_players.iter().enumerate() {
                    // If they are in pickup distance...
                    if distance(object.pos, *pos) < ITEM_PICKUP_DISTANCE.into() {
                        // And have avalable hotbar space...
                        if let Some(slot) = data.inventory.hotbar_empty_space() {
                            // Remove entity from every player
                            for player in server_players {
                                outgoing.push((Packet::RemoveObject(object.uuid), *ips.get(&player.0).expect("No IP found for a user connected to a server")));
                            }
                            // Add item to hotbar
                            server.data.players[index].2.inventory.hotbar[slot] = item;
                            // Tell user they have a new item
                            outgoing.push((Packet::InventoryState(server.data.players[index].2.inventory.clone()), *ips.get(&server.data.players[index].0).expect("No IP found for a user connected to a server")));
                            // Remove entity from server data
                            server.data.objects.remove(object_index - removed);
                            removed += 1;
                            continue 'object;
                        }
                    }
                }
                // Item magnet
                // reinit for new ref
                let server_players = &server.data.players;
                // If not picked up, for every player...
                for (_user, pos, data) in server_players.iter() {
                    // If they are in magnet distance...
                    if distance(object.pos, *pos) < ITEM_MAGNET_DISTANCE.into() {
                        // And have avalable hotbar space...
                        if let Some(_slot) = data.inventory.hotbar_empty_space() {
                            // dtotal=√((x_2-x_1)²+(y_2-y_1)²)
                            let dx = pos.translation.x - object.pos.translation.x;
                            let dy = pos.translation.y - object.pos.translation.y;
                            let dtotal = ((dx.powi(2))+(dy.powi(2))).sqrt();
                            let Δ = 64.0 / (((dtotal.powi(2)) + 100.0).sqrt());
                            let Δx = Δ*(dx/dtotal);
                            let Δy = Δ*(dy/dtotal);
                            let new_pos = Transform::from_xyz(object.pos.translation.x + Δx, object.pos.translation.y + Δy, 0.0);
                            let mut new_object = object.clone();
                            new_object.pos = new_pos;
                            // Update entity for every player
                            for player in server_players {
                                outgoing.push((Packet::UpdateObject(new_object.clone()), *ips.get(&player.0).expect("No IP found for a user connected to a server")));
                            }
                            // Update entity on the server side
                            server.data.objects[object_index - removed].pos = new_pos;
                            break;
                        }
                    }
                }
            }
        }
    }
    outgoing
}

fn save_world(save: SaveGame) {
    let enc = bincode::serialize(&save).expect("Unable to serialize a SaveGame.");
    std::fs::write(save.path, enc).expect("Unable to write a SaveGame to disk.");
}

pub fn save_folder() -> std::path::PathBuf {
    let mut dir = std::env::current_dir().expect("Unable to access the current directory.");
    dir.push("saves");
    std::fs::create_dir_all(dir.clone()).expect("Unable to create required directories.");
    dir
}

/// Returns a `PathBuf` to the folder used for storing profiles.
pub fn profile_folder() -> std::path::PathBuf {
    let mut dir = std::env::current_dir().expect("Unable to access the current directory.");
    dir.push("users");
    std::fs::create_dir_all(dir.clone()).expect("Unable to create required directories.");
    dir
}

/// Saves a `Profile` to the disk.
fn save_profile(profile: Profile) {
    // Encode profile
    let enc = bincode::serialize(&profile).expect("Unable to serialize a Profile.");

    // Get appropriate path and name
    let mut path = profile_folder();
    path.push(format!("{}{}.bic", profile.user.username, profile.user.tag));

    // Save to disk
    std::fs::write(path, enc).expect("Unable to write a profile to the disk.");
}
