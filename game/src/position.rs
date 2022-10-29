use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub rotation: Quat,
    pub vector: Vec3,
}

pub fn sync_transform_system(mut query: Query<(&mut Transform, &Position)>) {
    query.for_each_mut(|item| {
        let (mut transform, position) = item;
        transform.translation = position.vector;
        transform.rotation = position.rotation;
    })
}
