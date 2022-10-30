use crate::movement::{DestroyWhenOutOfBounds, MovementInput, MovementSpeeds};
use crate::player::PlayerCharacter;
use crate::position::Position;
use crate::weapons::Weapon;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Missile {}

pub fn player_shoot_missile_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerCharacter, &mut Weapon, &Position)>,
) {
    let (_, mut weapon, position) = query.single_mut();

    if keyboard_input.pressed(KeyCode::Space) {
        if weapon.remaining_reload_time > 0.0 {
            return;
        }

        weapon.remaining_reload_time = weapon.reload_speed;

        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform::from_translation(position.vector)
                    .with_rotation(position.rotation)
                    .with_scale(Vec3::splat(weapon.size)),
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
                position: weapon.projectile_speed,
                rotation: 0.0,
            })
            .insert(DestroyWhenOutOfBounds {})
            .insert(Collider::cuboid(weapon.size, weapon.size))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(RigidBody::Dynamic)
            .insert(Velocity {
                linvel: Vec2::new(1.0, 2.0),
                angvel: 0.2,
            })
            .insert(ActiveEvents::CONTACT_FORCE_EVENTS);
    }
}

pub fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
