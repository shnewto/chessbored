use bevy::{prelude::*, window::PresentMode};
use bevy_mod_picking::*;

mod assets;
mod board;
mod camera;
mod pieces;
mod state;

pub fn main() {
    let clear_color_hex_string = "69696b";
    App::new()
        .insert_resource(assets::BoardAssets::default())
        .insert_resource(board::Board::default())
        .insert_resource(WindowDescriptor {
            width: 720.,
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
        .add_state(state::ChessState::Setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        // .add_plugin(DebugEventsPickingPlugin)
        .add_system_set(SystemSet::on_update(state::ChessState::Setup).with_system(setup))
        .add_system_set(
            SystemSet::on_enter(state::ChessState::Loading).with_system(assets::load_assets),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Loading).with_system(assets::check_assets),
        )
        .add_system_set(
            SystemSet::on_exit(state::ChessState::Loading)
                .with_system(board::setup_board)
                .label("setup_piece_selection"),
        )
        .add_system_set(
            SystemSet::on_exit(state::ChessState::Loading)
                .with_system(pieces::setup_piece_selection)
                .after("setup_piece_selection")
                .label("piece_selection"),
        )
        .add_system_set(
            SystemSet::on_exit(state::ChessState::Loading)
                .with_system(camera::setup)
                .after("piece_selection"),
        )
        .add_system_to_stage(CoreStage::PostUpdate, pieces::selection)
        .add_system_to_stage(CoreStage::PostUpdate, pieces::side_piece_selection)
        .add_system_to_stage(CoreStage::Last, pieces::cancel_piece_movement)
        .add_system_to_stage(CoreStage::Last, pieces::piece_movement)
        .add_system_to_stage(CoreStage::Last, pieces::clear_board)
        .add_system_to_stage(CoreStage::Last, pieces::starting_positions)
        .run();
}

fn setup(mut board: ResMut<board::Board>, mut state: ResMut<State<state::ChessState>>) {
    *board = board::board_map();
    state.set(state::ChessState::Loading).unwrap();
}
