use crate::movement::{DestroyWhenOutOfBounds, MovementInput, MovementSpeeds};
use crate::player::PlayerCharacter;
use crate::position::Position;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
pub struct Missile {}

pub fn player_shoot_missile_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&PlayerCharacter, &Position)>,
) {
    let (_, position) = query.single();

    if keyboard_input.pressed(KeyCode::Space) {
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform::from_translation(position.vector)
                    .with_rotation(position.rotation)
                    .with_scale(Vec3::splat(4.)),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                ..default()
            })
            .insert(Missile {})
            .insert(Position {
                rotation: position.rotation,
                vector: position.vector,
            })
            .insert(MovementSpeeds {
                movement_speed: 1.0,
                rotation_speed: 1.0,
            })
            .insert(MovementInput {
                position: 700.0,
                rotation: 0.0,
            })
            .insert(DestroyWhenOutOfBounds {});
    }
}
