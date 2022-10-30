mod enemy;
mod missile;
mod movement;
mod player;
mod position;
mod weapons;

use bevy_rapier2d::prelude::*;

use crate::enemy::setup_enemy;
use crate::missile::{display_events, player_shoot_missile_system};
use crate::movement::{
    bounded_movement_system, destroy_bounded_movement_system, mark_out_of_bounds_system,
    movement_system,
};
use crate::player::{player_movement_input_system, setup_player};
use crate::position::sync_transform_system;
use crate::weapons::weapon_reload_system;
use bevy::window::PresentMode;
use bevy::{prelude::*, time::FixedTimestep};

pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Really fun game!".to_string(),
            width: 1200.,
            height: 640.,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_player)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_enemy)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement_input_system)
                .with_system(player_shoot_missile_system)
                .with_system(movement_system)
                .with_system(bounded_movement_system)
                .with_system(destroy_bounded_movement_system.exclusive_system())
                .with_system(mark_out_of_bounds_system)
                .with_system(weapon_reload_system)
                .with_system(display_events)
                .with_system(sync_transform_system),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
