use crate::movement::{DestroyWhenOutOfBounds, MovementInput, MovementSpeeds};
use crate::player::PlayerCharacter;
use crate::position::Position;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use std::ops::Add;

#[derive(Component)]
pub struct Missile {}

#[derive(Component)]
pub struct Collidable {}

pub fn missile_collision_system(
    mut query: Query<(&Collidable, &Position, Without<Missile>)>,
    missiles: Query<(&Missile, &Position, &Collidable)>,
) {
    query.for_each_mut(|item| {
        missiles.for_each(|missile| {
            let (_, missile_position, _) = missile;
            let (_, item_position, _) = item;

            if item_position
                .vector
                .add(Vec3::splat(1.0))
                .cmpgt(missile_position.vector.add(Vec3::splat(0.5)))
                .any()
                && item_position
                    .vector
                    .add(Vec3::splat(-1.0))
                    .cmplt(missile_position.vector.add(Vec3::splat(-0.5)))
                    .any()
            {
                println!("collision");
            }
        })
    })
}

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
            .insert(DestroyWhenOutOfBounds {})
            .insert(Collidable {});
    }
}
