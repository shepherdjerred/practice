use crate::movement::{ConstrainToBounds, MovementInput, MovementSpeeds};
use crate::position::Position;
use crate::weapons::Weapon;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCharacter {}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .insert(PlayerCharacter {})
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
            rotation: 0.0,
        })
        .insert(ConstrainToBounds {})
        .insert(Weapon {
            projectile_speed: 200.0,
            size: 4.0,
            reload_speed: 50.0,
            remaining_reload_time: 0.0,
        });
}

pub fn player_movement_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerCharacter, &mut MovementInput)>,
) {
    let (_, mut input) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        movement_factor -= 0.3;
    }

    input.position = movement_factor;
    input.rotation = rotation_factor;
}
