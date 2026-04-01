mod collision;
mod defs;
mod messages;
mod network;

use defs::*;
use raylib::prelude::*;
use std::collections::HashMap;

use collision::PlayerMovement;
use collision::resolve_player_collisions;
use messages::handle_msgs;

use shared::network::{ClientMessage, Vec2};

fn main() {
    let (mut rl, thread) = raylib::init().size(1280, 720).title("Platformer").build();

    rl.set_target_fps(60);
    let size = Vector2::new(50.0, 50.0);
    let mut player = Player {
        pos: Vector2::new(0.0, -100.0),
        vel: Vector2::zero(),
        mouse: Vector2::zero(),
    };
    let mut enemies: HashMap<u32, PlayerInterpolater> = HashMap::new();
    let mut platforms = Vec::new();
    for i in 0..10 {
        platforms.push(Platform {
            rect: Rectangle::new((i as f32 * 1000.0) - 400.0, 250.0 - 25.0, 800.0, 50.0),
        });
    }

    let mut camera = Camera2D {
        target: player.pos,
        offset: Vector2::new(1280.0 / 2.0, 720.0 / 2.0),
        rotation: 0.0,
        zoom: 1.0,
    };

    let (tx, mut rx) = network::spawn_client();

    let mut my_id: Option<u32> = None;
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        handle_msgs(&mut my_id, &mut enemies, &mut rx);

        let pmove = PlayerMovement {
            left: rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT),
            right: rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT),
            up: rl.is_key_down(KeyboardKey::KEY_SPACE) || rl.is_key_down(KeyboardKey::KEY_W),
        };
        resolve_player_collisions(&rl, &mut player, &size, &pmove, &platforms, dt);

        camera.target = player.pos;

        let mouse_pos = rl.get_mouse_position();
        let world_mouse = rl.get_screen_to_world2D(mouse_pos, camera);

        player.mouse = world_mouse;

        let _ = tx.send(ClientMessage::PositionUpdate {
            position: Vec2::new(player.pos.x, player.pos.y),
        });
        let _ = tx.send(ClientMessage::MouseUpdate {
            position: Vec2::new(player.mouse.x, player.mouse.y),
        });

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d2 = d.begin_mode2D(camera);

            for p in &platforms {
                d2.draw_rectangle_rec(&p.rect, Color::new(51, 204, 51, 255));
            }

            d2.draw_rectangle(
                (player.pos.x - size.x / 2.0) as i32,
                (player.pos.y - size.y / 2.0) as i32,
                size.x as i32,
                size.y as i32,
                Color::new(204, 51, 51, 255),
            );
            d2.draw_circle(
                player.mouse.x as i32,
                player.mouse.y as i32,
                15.0,
                Color::new(255, 255, 0, 128),
            );
            for (id, enemy) in &enemies {
                // if let Some(my_id) = my_id
                //     && my_id == *id
                // {
                //     continue;
                // }
                let interpolated = enemy.interpolate();
                d2.draw_rectangle(
                    (interpolated.pos.x - size.x / 2.0) as i32,
                    (interpolated.pos.y - size.y / 2.0) as i32,
                    size.x as i32,
                    size.y as i32,
                    Color::new(51, 51, 204, 255),
                );

                d2.draw_circle(
                    interpolated.mouse.x as i32,
                    interpolated.mouse.y as i32,
                    15.0,
                    Color::new(255, 0, 255, 128),
                );
            }
        }
    }
}
