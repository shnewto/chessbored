use crate::state::ChessState;
use bevy::{asset::LoadState, prelude::*};

#[derive(Component, Debug, Default)]
pub struct BoardAssets {
    pub dark_square_handle: Handle<Image>,
    pub light_square_handle: Handle<Image>,
    pub bp: Handle<Image>,
    pub br: Handle<Image>,
    pub bn: Handle<Image>,
    pub bb: Handle<Image>,
    pub bq: Handle<Image>,
    pub bk: Handle<Image>,
    pub wp: Handle<Image>,
    pub wr: Handle<Image>,
    pub wn: Handle<Image>,
    pub wb: Handle<Image>,
    pub wq: Handle<Image>,
    pub wk: Handle<Image>,
}

pub fn load_assets(asset_server: Res<AssetServer>, mut board_assets: ResMut<BoardAssets>) {
    board_assets.dark_square_handle = asset_server.load("board/dark-square.png");
    board_assets.light_square_handle = asset_server.load("board/light-square.png");
    board_assets.bp = asset_server.load("pieces/bp.png");
    board_assets.br = asset_server.load("pieces/br.png");
    board_assets.bn = asset_server.load("pieces/bn.png");
    board_assets.bb = asset_server.load("pieces/bb.png");
    board_assets.bq = asset_server.load("pieces/bq.png");
    board_assets.bk = asset_server.load("pieces/bk.png");

    board_assets.wp = asset_server.load("pieces/wp.png");
    board_assets.wr = asset_server.load("pieces/wr.png");
    board_assets.wn = asset_server.load("pieces/wn.png");
    board_assets.wb = asset_server.load("pieces/wb.png");
    board_assets.wq = asset_server.load("pieces/wq.png");
    board_assets.wk = asset_server.load("pieces/wk.png");
}

pub fn check_assets(
    mut state: ResMut<State<ChessState>>,
    asset_server: Res<AssetServer>,
    board_assets: Res<BoardAssets>,
) {
    if let (
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
    ) = (
        asset_server.get_load_state(&board_assets.dark_square_handle),
        asset_server.get_load_state(&board_assets.light_square_handle),
        asset_server.get_load_state(&board_assets.bp),
        asset_server.get_load_state(&board_assets.br),
        asset_server.get_load_state(&board_assets.bn),
        asset_server.get_load_state(&board_assets.bb),
        asset_server.get_load_state(&board_assets.bq),
        asset_server.get_load_state(&board_assets.bk),
        asset_server.get_load_state(&board_assets.wp),
        asset_server.get_load_state(&board_assets.wr),
        asset_server.get_load_state(&board_assets.wn),
        asset_server.get_load_state(&board_assets.wb),
        asset_server.get_load_state(&board_assets.wq),
        asset_server.get_load_state(&board_assets.wk),
    ) {
        state.set(ChessState::Loaded).unwrap();
    }
}
