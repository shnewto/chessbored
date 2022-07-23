use bevy::prelude::*;
use bevy_mod_picking::*;

/// Used to help identify our main camera
#[derive(Component)]
pub struct ChessCamera;

pub fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_xyz(175.0, 175.0, 10.0);

    commands
        .spawn()
        .insert_bundle(camera)
        .insert_bundle(PickingCameraBundle::default())
        .insert(ChessCamera);
}

pub fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}
