use bevy::{prelude::*, window::PresentMode};
use bevy_egui::EguiPlugin;
// use bevy_egui::EguiPlugin;
use bevy_mod_picking::*;
use fen::SavedFenState;
use pieces::PieceMaterialHandles;

mod assets;
mod board;
mod camera;
mod fen;
mod pieces;
mod state;
mod tips;
mod types;

pub fn main() {
    let clear_color_hex_string = "69696b";
    App::new()
        .insert_resource(assets::BoardAssets::default())
        .insert_resource(assets::TextAssets::default())
        .insert_resource(types::Board::default())
        .insert_resource(SavedFenState::default())
        .insert_resource(PieceMaterialHandles::default())
        .insert_resource(WindowDescriptor {
            width: 680.,
            height: 700.,
            title: "chessbored".to_string(),
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
        .add_plugin(EguiPlugin)
        // .add_plugin(DebugEventsPickingPlugin)
        .add_system_set(SystemSet::on_update(state::ChessState::Setup).with_system(setup))
        .add_system_set(
            SystemSet::on_enter(state::ChessState::Loading).with_system(assets::load_assets),
        )
        .add_system_set(
            SystemSet::on_exit(state::ChessState::Loading)
                .with_system(board::setup_board)
                .label("setup_board"),
        )
        .add_system_set(
            SystemSet::on_exit(state::ChessState::Loading)
                .with_system(pieces::set_sprite_handles)
                .label("piece_sprite_handles")
                .after("setup_board"),
        )
        .add_system_set(
            SystemSet::on_exit(state::ChessState::Loading)
                .with_system(pieces::setup_piece_selection)
                .after("piece_sprite_handles")
                .label("piece_selection"),
        )
        .add_system_set(
            SystemSet::on_exit(state::ChessState::Loading)
                .with_system(camera::spawn_ui_camera)
                .label("spawn_ui_camera")
                .after("piece_selection"),
        )
        .add_system_set(
            SystemSet::on_exit(state::ChessState::Loading)
                .with_system(fen::spawn)
                .label("fen")
                .after("spawn_ui_camera"),
        )
        .add_system_set(
            SystemSet::on_exit(state::ChessState::Loading)
                .with_system(tips::spawn)
                .label("tips")
                .after("fen"),
        )
        .add_system_set(
            SystemSet::on_exit(state::ChessState::Loading)
                .with_system(camera::setup)
                .after("tips"),
        )
        .add_system_to_stage(CoreStage::Update, pieces::cancel_piece_movement)
        .add_system_to_stage(CoreStage::Update, pieces::starting_positions)
        .add_system_to_stage(CoreStage::Update, pieces::selection)
        .add_system_to_stage(CoreStage::Update, pieces::side_piece_selection)
        .add_system_to_stage(CoreStage::Update, fen::generate_fen)
        .add_system_to_stage(CoreStage::Update, fen::toggle_save_position)
        .add_system_to_stage(CoreStage::Update, fen::populate_board_from_fen)
        .add_system_to_stage(CoreStage::Update, fen::copy_to_clipboard)
        .add_system_to_stage(CoreStage::PostUpdate, pieces::clear_board)
        .add_system_to_stage(CoreStage::Last, pieces::piece_movement)
        .run();
}

fn setup(mut board: ResMut<types::Board>, mut state: ResMut<State<state::ChessState>>) {
    *board = board::board_map();
    state.set(state::ChessState::Loading).unwrap();
}
