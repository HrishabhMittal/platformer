use raylib::math::{Rectangle, Vector2};
pub struct Player {
    pub pos: Vector2,
    pub vel: Vector2,
    pub size: Vector2,
}

pub struct Platform {
    pub rect: Rectangle,
}
