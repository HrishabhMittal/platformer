mod collision;
mod defs;

use collision::resolve_player_collisions;
use defs::*;
use raylib::prelude::*;


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Raylib Platformer")
        .build();

    rl.set_target_fps(60);

    let mut player = Player {
        pos: Vector2::new(0.0, -100.0),
        vel: Vector2::zero(),
        size: Vector2::new(50.0, 50.0),
    };

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

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        resolve_player_collisions(&rl, &mut player, &platforms, dt);

        camera.target = player.pos;

        let mouse_pos = rl.get_mouse_position();
        let world_mouse = rl.get_screen_to_world2D(mouse_pos, camera);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d2 = d.begin_mode2D(camera);

            for p in &platforms {
                d2.draw_rectangle_rec(&p.rect, Color::new(51, 204, 51, 255));
            }

            d2.draw_rectangle(
                (player.pos.x - player.size.x / 2.0) as i32,
                (player.pos.y - player.size.y / 2.0) as i32,
                player.size.x as i32,
                player.size.y as i32,
                Color::new(204, 51, 51, 255),
            );

            d2.draw_circle(
                world_mouse.x as i32,
                world_mouse.y as i32,
                15.0,
                Color::new(255, 255, 0, 128),
            );
        }
    }
}
