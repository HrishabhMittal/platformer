use avian2d::prelude::*;
use bevy::prelude::*;

use crate::player::{CursorCircle, Player};
use crate::camera::MainCamera;

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.8, 0.2),
            custom_size: Some(Vec2::new(800.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -250.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(800.0, 50.0),
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 100.0, 0.0),
        Player,
        RigidBody::Dynamic,
        Collider::rectangle(50.0, 50.0),
        LockedAxes::ROTATION_LOCKED,
        ConstantLinearAcceleration(Vec2::ZERO),
    ));
}

pub fn setup_cursor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(15.0))),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 0.0, 0.5))),
        Transform::from_xyz(0.0, 0.0, 10.0),
        CursorCircle,
    ));
}
