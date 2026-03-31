use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    PositionUpdate { position: Vec2 },
    MouseUpdate { position: Vec2 },
}
#[derive(Debug)]
pub struct ClientObject {
    pub id: u32,
    pub client_message: ClientMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    NotifyId { id: u32 },
    PlayerMoved { id: u32, position: Vec2 },
    PlayerMouseMoved { id: u32, position: Vec2 },
    PlayerJoined { id: u32 },
    PlayerLeft { id: u32 },
}
