use bevy::{prelude::*, window::PresentMode};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::*;
use fen::SavedFenState;
use pieces::PieceMaterialHandles;

mod assets;
mod board;
mod camera;
mod control_ux;
mod fen;
mod pieces;
mod state;
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
        // .add_plugin(DebugCursorPickingPlugin)
        // .add_plugin(DebugEventsPickingPlugin)
        .add_system_set(SystemSet::on_update(state::ChessState::Setup).with_system(setup))
        .add_system_set(
            SystemSet::on_update(state::ChessState::Loading).with_system(assets::load_assets),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Loaded)
                .with_system(board::setup_board)
                .label("setup_board"),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Loaded)
                .with_system(pieces::set_sprite_handles)
                .label("piece_sprite_handles")
                .after("setup_board"),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Loaded)
                .with_system(pieces::setup_piece_selection)
                .after("piece_sprite_handles")
                .label("piece_selection"),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Loaded)
                .with_system(fen::spawn)
                .label("fen")
                .after("piece_selection"),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Loaded)
                .with_system(control_ux::spawn)
                .label("control_ux")
                .after("fen"),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Loaded)
                .with_system(camera::setup)
                .after("control_ux"),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Running)
                .with_system(pieces::cancel_piece_movement)
                .label("cancel_piece_movement"),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Running)
                .with_system(pieces::starting_positions)
                .label("starting_positions")
                .after("cancel_piece_movement"),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Running)
                .with_system(pieces::selection)
                .label("selection")
                .after("starting_positions"),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Running)
                .with_system(pieces::side_piece_selection)
                .label("side_piece_selection")
                .after("selection"),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Running)
                .with_system(pieces::drop_piece)
                .label("drop_piece")
                .after("side_piece_selection"),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Running)
                .with_system(fen::generate_fen)
                .label("generate_fen")
                .after("drop_piece"),
        )
        // .add_system_set(
        //     SystemSet::on_update(state::ChessState::Running)
        //         .with_system(fen::toggle_save_position)
        //         .label("toggle_save_position")
        //         .after("generate_fen"),
        // )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Running)
                .with_system(fen::populate_board_from_fen)
                .label("populate_board_from_fen")
                .after("generate_fen"),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Running)
                .with_system(fen::copy_to_clipboard)
                .label("copy_to_clipboard")
                .after("populate_board_from_fen"),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Running)
                .with_system(pieces::clear_board)
                .label("clear_board")
                .after("copy_to_clipboard"),
        )
        .add_system_set(
            SystemSet::on_update(state::ChessState::Running)
                .with_system(pieces::piece_movement)
                .label("piece_movement")
                .after("clear_board"),
        )
        .run();
}

fn setup(mut board: ResMut<types::Board>, mut state: ResMut<State<state::ChessState>>) {
    *board = board::board_map();
    state.set(state::ChessState::Loading).unwrap();
}
