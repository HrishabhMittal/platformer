use crate::player::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;
pub fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    const LAG: f32 = 0.2;
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            camera_transform.translation.x +=
                (player_transform.translation.x - camera_transform.translation.x) * LAG;
            camera_transform.translation.y +=
                (player_transform.translation.y - camera_transform.translation.y) * LAG;
        }
    }
}
