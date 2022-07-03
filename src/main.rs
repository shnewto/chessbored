use bevy::{prelude::*, window::PresentMode};
use bevy_mod_picking::*;
use board::{board_map, check_board, load_board, Board, BoardAssets};
use pieces::{
    check_pieces, load_piece_assets, piece_movement, piece_selection, spawn_pieces, PieceAssets,
};
use state::ChessState;

mod board;
mod camera;
mod pieces;
mod state;

pub fn main() {
    let clear_color_hex_string = "69696b";
    App::new()
        .insert_resource(BoardAssets::default())
        .insert_resource(PieceAssets::default())
        .insert_resource(Board::default())
        .insert_resource(WindowDescriptor {
            width: 640.,
            height: 640.,
            title: "chessboard".to_string(),
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(ClearColor(
            Color::hex(clear_color_hex_string).unwrap_or_else(|_| {
                panic!("couldn't make hex color from {}", clear_color_hex_string)
            }),
        ))
        .add_state(ChessState::Setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        // .add_plugin(DebugEventsPickingPlugin)
        .add_system_set(SystemSet::on_update(ChessState::Setup).with_system(setup))
        .add_system_set(SystemSet::on_enter(ChessState::LoadingBoard).with_system(load_board))
        .add_system_set(SystemSet::on_update(ChessState::LoadingBoard).with_system(check_board))
        .add_system_set(
            SystemSet::on_enter(ChessState::LoadingPieces)
                .with_system(load_piece_assets)
                .label("load_piece_assets"),
        )
        .add_system_set(
            SystemSet::on_enter(ChessState::LoadingPieces)
                .with_system(spawn_pieces)
                .after("load_piece_assets"),
        )
        .add_system_set(SystemSet::on_update(ChessState::LoadingPieces).with_system(check_pieces))
        .add_system_set(SystemSet::on_exit(ChessState::LoadingPieces).with_system(camera::setup))
        .add_system_to_stage(CoreStage::PostUpdate, piece_selection)
        .add_system_to_stage(CoreStage::Last, piece_movement)
        .run();
}

fn setup(mut board: ResMut<Board>, mut state: ResMut<State<ChessState>>) {
    *board = board_map();
    state.set(ChessState::LoadingBoard).unwrap();
}
