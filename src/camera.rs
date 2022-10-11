use bevy::prelude::*;
use bevy_mod_picking::*;

use crate::state::ChessState;

#[derive(Component)]
pub struct ChessCamera;

pub fn setup(mut commands: Commands, mut state: ResMut<State<ChessState>>) {
    commands
        .spawn()
        .insert_bundle(Camera2dBundle {
            transform: Transform::from_xyz(175.0, 175.0, 999.9),
            ..default()
        })
        .insert_bundle(PickingCameraBundle::default())
        .insert(ChessCamera);

    state.set(ChessState::Running).unwrap();
}
