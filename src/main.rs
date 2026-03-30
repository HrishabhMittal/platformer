use avian2d::prelude::*;
use bevy::prelude::*;

mod player;
mod camera;
mod setup;
mod constants;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(Gravity(Vec2::new(0.0, -constants::GRAVITY)))
        .add_systems(Startup, (setup::setup, setup::setup_cursor))
        .add_systems(FixedUpdate, player::player_movement)
        .add_systems(Update,player::update_cursor)
        .add_systems(Update,camera::camera_follow)
        .run();
}
