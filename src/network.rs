use bevy::prelude::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    PositionUpdate { position: Vec2 },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    PlayerMoved { id: u32, position: Vec2 },
    PlayerJoined { id: u32 },
    PlayerLeft { id: u32 },
}
