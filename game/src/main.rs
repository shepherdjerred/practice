mod enemy;
mod missile;
mod movement;
mod player;
mod position;

use crate::enemy::setup_enemy;
use crate::missile::{missile_collision_system, player_shoot_missile_system};
use crate::movement::{
    bounded_movement_system, destroy_bounded_movement_system, mark_out_of_bounds_system,
    movement_system,
};
use crate::player::{player_movement_input_system, setup_player};
use crate::position::sync_transform_system;
use bevy::{prelude::*, time::FixedTimestep};

pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
                .with_system(missile_collision_system)
                .with_system(sync_transform_system),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
