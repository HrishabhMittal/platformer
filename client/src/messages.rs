use crate::defs::*;
use raylib::prelude::*;
use std::collections::HashMap;

use shared::network::ServerMessage;
use tokio::sync::mpsc;
pub fn handle_msgs(
    my_id: &mut Option<u32>,
    enemies: &mut HashMap<u32, PlayerInterpolater>,
    rx: &mut mpsc::Receiver<ServerMessage>,
) {
    while let Ok(msg) = rx.try_recv() {
        match msg {
            ServerMessage::NotifyId { id } => {
                *my_id = Some(id);
            }
            ServerMessage::PlayerMoved { id, position } => {
                // if Some(id) != *my_id {
                    let pos = Vector2::new(position.x, position.y);
                    let enemy = enemies.entry(id).or_insert_with(|| {
                        PlayerInterpolater::from(Player {
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
