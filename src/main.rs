use bevy::prelude::*;
use bevy_mod_picking::*;
use board::{board_map, check_board, load_board, Board, BoardAssets};
use pieces::{check_pieces, PieceAssets, load_piece_assets, spawn_pieces};
use state::ChessState;

mod board;
mod camera;
mod pieces;
mod state;

pub fn main() {
    App::new()
        .insert_resource(BoardAssets::default())
        .insert_resource(PieceAssets::default())
        .insert_resource(Board::default())
        .add_state(ChessState::Setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(DebugEventsPickingPlugin)
        .add_system_set(SystemSet::on_update(ChessState::Setup).with_system(setup))
        .add_system_set(SystemSet::on_enter(ChessState::LoadingBoard).with_system(load_board))
        .add_system_set(SystemSet::on_update(ChessState::LoadingBoard).with_system(check_board))
        .add_system_set(SystemSet::on_enter(ChessState::LoadingPieces).with_system(load_piece_assets).label("load_piece_assets"))
        .add_system_set(SystemSet::on_enter(ChessState::LoadingPieces).with_system(spawn_pieces).after("load_piece_assets"))
        .add_system_set(SystemSet::on_update(ChessState::LoadingPieces).with_system(check_pieces))
        .add_system_set(SystemSet::on_exit(ChessState::LoadingPieces).with_system(camera::setup))
        .add_system_to_stage(CoreStage::PostUpdate, print_events)
        .run();
}

fn setup(mut board: ResMut<Board>, mut state: ResMut<State<ChessState>>) {
    *board = board_map();
    state.set(ChessState::LoadingBoard).unwrap();
}

pub fn print_events(mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("PickingEvent::Selection: {:?}", e),
            PickingEvent::Hover(e) => info!("PickingEvent::Hover: {:?}", e),
            PickingEvent::Clicked(e) => info!("PickingEvent::Clicked: {:?}", e),
        }
    }
}