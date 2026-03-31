use crate::defs::*;
use raylib::prelude::*;
use shared::constants::{GRAVITY, MAX_VELOCITY};
pub struct PlayerMovement {
    pub left: bool,
    pub right: bool,
    pub up: bool,
}
pub fn resolve_player_collisions(
    rl: &RaylibHandle,
    player: &mut Player,
    size: &Vector2,
    pmove: &PlayerMovement,
    platforms: &Vec<Platform>,
    dt: f32,
) {
    let accel_rate = 5000.0;
    let jump_force = 500.0;
    let friction = 0.1;


    let mut accel = Vector2::new(0.0, GRAVITY);
    
    if pmove.left && !pmove.right {
        accel.x = -accel_rate * (1.0 - player.vel.x.abs() / MAX_VELOCITY);
    } else if pmove.right && !pmove.left {
        accel.x = accel_rate * (1.0 - player.vel.x / MAX_VELOCITY);
    } else {
        player.vel.x *= 1.0 - friction;
    }

    if pmove.up {
        accel.y = -jump_force * (1.0 - (-player.vel.y / MAX_VELOCITY));
    }

    player.vel += accel * dt;
    if player.vel.y > MAX_VELOCITY * 3.0 {
        player.vel.y = MAX_VELOCITY * 3.0;
    }
    if player.vel.y < -MAX_VELOCITY * 3.0 {
        player.vel.y = -MAX_VELOCITY * 3.0;
    }

    player.pos.x += player.vel.x * dt;
    let mut player_rect = Rectangle::new(
        player.pos.x - size.x / 2.0,
        player.pos.y - size.y / 2.0,
        size.x,
        size.y,
    );

    for p in platforms {
        if p.rect.check_collision_recs(&player_rect) {
            if player.vel.x > 0.0 {
                player.pos.x = p.rect.x - (size.x / 2.0);
            } else if player.vel.x < 0.0 {
                player.pos.x = p.rect.x + p.rect.width + (size.x / 2.0);
            }
            player.vel.x = 0.0;
        }
    }

    player.pos.y += player.vel.y * dt;
    player_rect.x = player.pos.x - size.x / 2.0;
    player_rect.y = player.pos.y - size.y / 2.0;

    for p in platforms {
        if p.rect.check_collision_recs(&player_rect) {
            if player.vel.y > 0.0 {
                player.pos.y = p.rect.y - (size.y / 2.0);
            } else if player.vel.y < 0.0 {
                player.pos.y = p.rect.y + p.rect.height + (size.y / 2.0);
            }
            player.vel.y = 0.0;
        }
    }
    let accel_rate = 5000.0;
    let jump_force = 500.0;
    let friction = 0.1;

    let key_left = rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT);
    let key_right = rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT);
    let booster = rl.is_key_down(KeyboardKey::KEY_SPACE) || rl.is_key_down(KeyboardKey::KEY_W);

    let mut accel = Vector2::new(0.0, GRAVITY);

    if key_left && !key_right {
        accel.x = -accel_rate * (1.0 - player.vel.x.abs() / MAX_VELOCITY);
    } else if key_right && !key_left {
        accel.x = accel_rate * (1.0 - player.vel.x / MAX_VELOCITY);
    } else {
        player.vel.x *= 1.0 - friction;
    }

    if booster {
        accel.y = -jump_force * (1.0 - (-player.vel.y / MAX_VELOCITY));
    }

    player.vel += accel * dt;
    if player.vel.y > MAX_VELOCITY * 3.0 {
        player.vel.y = MAX_VELOCITY * 3.0;
    }
    if player.vel.y < -MAX_VELOCITY * 3.0 {
        player.vel.y = -MAX_VELOCITY * 3.0;
    }

    player.pos.x += player.vel.x * dt;
    let mut player_rect = Rectangle::new(
        player.pos.x - size.x / 2.0,
        player.pos.y - size.y / 2.0,
        size.x,
        size.y,
    );

    for p in platforms {
        if p.rect.check_collision_recs(&player_rect) {
            if player.vel.x > 0.0 {
                player.pos.x = p.rect.x - (size.x / 2.0);
            } else if player.vel.x < 0.0 {
                player.pos.x = p.rect.x + p.rect.width + (size.x / 2.0);
            }
            player.vel.x = 0.0;
        }
    }

    player.pos.y += player.vel.y * dt;
    player_rect.x = player.pos.x - size.x / 2.0;
    player_rect.y = player.pos.y - size.y / 2.0;

    for p in platforms {
        if p.rect.check_collision_recs(&player_rect) {
            if player.vel.y > 0.0 {
                player.pos.y = p.rect.y - (size.y / 2.0);
            } else if player.vel.y < 0.0 {
                player.pos.y = p.rect.y + p.rect.height + (size.y / 2.0);
            }
            player.vel.y = 0.0;
        }
    }
}
