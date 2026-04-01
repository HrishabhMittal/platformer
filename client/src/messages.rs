use crate::defs::*;
use raylib::prelude::*;
use std::collections::HashMap;

use shared::network::ServerMessage;
use tokio::sync::mpsc;
pub fn handle_msgs(enemies: &mut HashMap<u32, Player>, rx: &mut mpsc::Receiver<ServerMessage>) {
    let mut my_id: Option<u32> = None;
    while let Ok(msg) = rx.try_recv() {
        match msg {
            ServerMessage::NotifyId { id } => {
                my_id = Some(id);
            }
            ServerMessage::PlayerMoved { id, position } => {
                if Some(id) != my_id {
                    let enemy = enemies.entry(id).or_insert(Player {
                        pos: Vector2::new(position.x, position.y),
                        vel: Vector2::zero(),
                        mouse: Vector2::zero(),
                    });
                    enemy.pos.x = position.x;
                    enemy.pos.y = position.y;
                }
            }
            ServerMessage::PlayerMouseMoved { id, position } => {
                if Some(id) != my_id {
                    if let Some(enemy) = enemies.get_mut(&id) {
                        enemy.mouse.x = position.x;
                        enemy.mouse.y = position.y;
                    }
                }
            }
            ServerMessage::PlayerLeft { id } => {
                enemies.remove(&id);
            }
            ServerMessage::PlayerJoined { id: _ } => {}
        }
    }
}
