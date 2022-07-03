use bevy::prelude::*;
use board::{board_map, check_board, load_board, Board, BoardAssets};
use pieces::{check_pieces, load_pieces, PieceAssets};
use state::ChessState;

mod board;
mod pieces;
mod state;

pub fn main() {
    App::new()
        .insert_resource(BoardAssets::default())
        .insert_resource(PieceAssets::default())
        .insert_resource(Board::default())
        .add_state(ChessState::Setup)
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_update(ChessState::Setup).with_system(setup))
        .add_system_set(SystemSet::on_enter(ChessState::LoadingBoard).with_system(load_board))
        .add_system_set(SystemSet::on_update(ChessState::LoadingBoard).with_system(check_board))
        .add_system_set(SystemSet::on_enter(ChessState::LoadingPieces).with_system(load_pieces))
        .add_system_set(SystemSet::on_update(ChessState::LoadingPieces).with_system(check_pieces))
        .add_system_set(SystemSet::on_exit(ChessState::LoadingPieces).with_system(loaded))
        .run();
}

fn setup(mut board: ResMut<Board>, mut state: ResMut<State<ChessState>>) {
    *board = board_map();
    state.set(ChessState::LoadingBoard).unwrap();
}

fn loaded(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_xyz(175.0, 175.0, 10.0);
    commands.spawn_bundle(camera);
}
