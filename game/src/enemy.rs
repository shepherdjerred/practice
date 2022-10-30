use crate::missile::Collidable;
use crate::movement::{ConstrainToBounds, MovementInput, MovementSpeeds};
use crate::position::Position;
use bevy::asset::AssetServer;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{default, Commands, Component, Res, SpriteBundle};

#[derive(Component)]
pub struct Enemy {}
pub fn setup_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_handle = asset_server.load("textures/ship_C.png");
    commands
        .spawn_bundle(SpriteBundle {
            texture: ship_handle,
            ..default()
        })
        .insert(MovementSpeeds {
            movement_speed: 500.0,
            rotation_speed: f32::to_radians(360.0),
        })
        .insert(Enemy {})
        .insert(Position {
            rotation: Quat::default(),
            vector: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 10.0,
            },
        })
        .insert(MovementInput {
            position: 0.0,
            rotation: 0.5,
        })
        .insert(ConstrainToBounds {})
        .insert(Collidable {});
}
