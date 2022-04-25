use std::net::SocketAddr;

use bevy::utils::HashMap;

use crate::{shared::{object::ObjectType, netty::Packet, saves::User}, consts::{ITEM_PICKUP_DISTANCE, ITEM_MAGNET_DISTANCE}, components::GamePosition};

use super::SaveGame;

#[allow(non_snake_case)]
pub fn tick(servers: &mut Vec<SaveGame>, ips: &HashMap<User, SocketAddr>) -> Vec<(Packet, SocketAddr)> {
    let mut outgoing: Vec<(Packet, SocketAddr)> = vec![];
    for server in servers {
        for (object_index, object) in server.data.objects.clone().iter().enumerate() {
            if let ObjectType::GroundItem(item) = object.rep {
                let server_players = &server.data.players;
                // Item pickup
                // For every player...
                let mut picked_up = false;
                for (index, (_user, pos, data)) in server_players.iter().enumerate() {
                    // If they are in pickup distance...
                    if object.pos.distance(*pos) < ITEM_PICKUP_DISTANCE.into() {
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
                            server.data.objects.remove(object_index);
                            picked_up = true;
                            break;
                        }
                    }
                }
                // Item magnet
                // reinit for new ref
                let server_players = &server.data.players;
                if !picked_up {
                    // If not picked up, for every player...
                    for (_user, pos, data) in server_players.iter() {
                        // If they are in magnet distance...
                        if object.pos.distance(*pos) < ITEM_MAGNET_DISTANCE.into() {
                            // And have avalable hotbar space...
                            if let Some(_slot) = data.inventory.hotbar_empty_space() {
                                let dx = pos.x - object.pos.x;
                                let dy = pos.x - object.pos.y;
                                let Δx = (1.0 / dx) * 20.0;
                                let Δy = (1.0 / dy) * 20.0;
                                let new_pos = GamePosition { x: object.pos.x + Δx, y: object.pos.y + Δy };
                                let mut new_object = object.clone();
                                new_object.pos = new_pos;
                                // Update entity for every player
                                for player in server_players {
                                    outgoing.push((Packet::UpdateObject(new_object.clone()), *ips.get(&player.0).expect("No IP found for a user connected to a server")));
                                }
                                // Update entity on the server side
                                server.data.objects[object_index].pos = new_pos;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    outgoing
}
