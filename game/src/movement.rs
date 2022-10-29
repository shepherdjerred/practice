use crate::position::Position;
use crate::{BOUNDS, TIME_STEP};
use bevy::prelude::*;

#[derive(Component)]
pub struct MovementSpeeds {
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

#[derive(Component)]
pub struct MovementInput {
    pub position: f32,
    pub rotation: f32,
}

#[derive(Component)]
pub struct ConstrainToBounds {}

#[derive(Component)]
pub struct DestroyWhenOutOfBounds {}

#[derive(Component)]
pub struct IsOutOfBounds {}

pub fn movement_system(mut query: Query<(&MovementSpeeds, &MovementInput, &mut Position)>) {
    query.for_each_mut(|item| {
        let (speeds, input, mut position) = item;
        position.rotation =
            Quat::from_rotation_z(input.rotation * speeds.rotation_speed * TIME_STEP)
                .mul_quat(position.rotation);

        let movement_direction = position.rotation * Vec3::Y;
        let movement_distance = input.position * speeds.movement_speed * TIME_STEP;
        let translation_delta = movement_direction * movement_distance;
        position.vector += translation_delta;
    })
}

pub fn bounded_movement_system(mut query: Query<(&ConstrainToBounds, &mut Position)>) {
    query.for_each_mut(|item| {
        let (_, mut position) = item;
        let extents = Vec3::from((BOUNDS / 2.0, 100.0));
        position.vector = position.vector.min(extents).max(-extents);
    })
}

pub fn mark_out_of_bounds_system(
    mut commands: Commands,
    mut query: Query<(Entity, &DestroyWhenOutOfBounds, &Position)>,
) {
    query.for_each_mut(|item| {
        let (entity, _, position) = item;
        let extents = Vec3::from((BOUNDS / 2.0, 100.0));
        if position.vector.cmpgt(extents).any() || position.vector.cmplt(-extents).any() {
            commands.entity(entity).insert(IsOutOfBounds {});
        }
    })
}

pub fn destroy_bounded_movement_system(
    mut commands: Commands,
    query: Query<Entity, With<IsOutOfBounds>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
        println!("destroyed entity")
    }
}
