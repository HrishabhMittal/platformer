use crate::defs::*;
use raylib::prelude::*;
use std::collections::HashMap;

use shared::{constants::MAX_HP, network::ServerMessage};
use tokio::sync::mpsc;
pub fn handle_msgs(
    my_id: &mut Option<u32>,
    enemies: &mut HashMap<u32, PlayerInterpolater>,
    rx: &mut mpsc::Receiver<ServerMessage>,
) {
    while let Ok(msg) = rx.try_recv() {
        match msg {
            ServerMessage::PlayerShot { id } => {
                let enemy = enemies.entry(id).or_insert_with(|| {
                    PlayerInterpolater::from(Player {
                        health: MAX_HP,
                        pos: Vector2::zero(),
                        vel: Vector2::zero(),
                        mouse: Vector2::zero(),
                    })
                });
                enemy.inc_shots();
            }
            ServerMessage::NotifyId { id } => {
                *my_id = Some(id);
            }
            ServerMessage::PlayerHealthChange { id, health } => {
                let enemy = enemies.entry(id).or_insert_with(|| {
                    PlayerInterpolater::from(Player {
                        health,
                        pos: Vector2::zero(),
                        vel: Vector2::zero(),
                        mouse: Vector2::zero(),
                    })
                });
                enemy.health_change(health);
                if health <= 0 {
                    enemies.remove(&id);
                }
            }
            ServerMessage::PlayerMoved { id, position } => {
                // if Some(id) != *my_id {
                let pos = Vector2::new(position.x, position.y);
                let enemy = enemies.entry(id).or_insert_with(|| {
                    PlayerInterpolater::from(Player {
                        health: MAX_HP,
                        pos,
                        vel: Vector2::zero(),
                        mouse: Vector2::zero(),
                    })
                });
                enemy.update_pos(pos);
                // }
            }
            ServerMessage::PlayerMouseMoved { id, position } => {
                // if Some(id) != *my_id {
                let mouse_pos = Vector2::new(position.x, position.y);
                let enemy = enemies.entry(id).or_insert_with(|| {
                    PlayerInterpolater::from(Player {
                        health: MAX_HP,
                        pos: Vector2::zero(),
                        vel: Vector2::zero(),
                        mouse: mouse_pos,
                    })
                });
                enemy.update_mouse(mouse_pos);
                // }
            }
            ServerMessage::PlayerLeft { id } => {
                enemies.remove(&id);
            }
            ServerMessage::PlayerJoined { id: _ } => {}
        }
    }
}
