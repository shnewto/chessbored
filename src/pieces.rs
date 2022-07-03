use bevy::{asset::LoadState, prelude::*};
use std::collections::HashMap;

use crate::{board::Board, state::ChessState};

#[derive(Component, Debug, Default)]
pub struct PieceAssets {
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

pub fn load_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut piece_assets: ResMut<PieceAssets>,
    board: Res<Board>,
) {
    piece_assets.bp = asset_server.load("pieces/bp.png");
    piece_assets.br = asset_server.load("pieces/br.png");
    piece_assets.bn = asset_server.load("pieces/bn.png");
    piece_assets.bb = asset_server.load("pieces/bq.png");
    piece_assets.bq = asset_server.load("pieces/bb.png");
    piece_assets.bk = asset_server.load("pieces/bk.png");

    piece_assets.wp = asset_server.load("pieces/wp.png");
    piece_assets.wr = asset_server.load("pieces/wr.png");
    piece_assets.wn = asset_server.load("pieces/wn.png");
    piece_assets.wb = asset_server.load("pieces/wb.png");
    piece_assets.wq = asset_server.load("pieces/wq.png");
    piece_assets.wk = asset_server.load("pieces/wk.png");

    commands.spawn_batch(pieces(&piece_assets, &board));
}

pub fn check_pieces(
    mut state: ResMut<State<ChessState>>,
    asset_server: Res<AssetServer>,
    piece_assets: Res<PieceAssets>,
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
    ) = (
        asset_server.get_load_state(&piece_assets.bp),
        asset_server.get_load_state(&piece_assets.br),
        asset_server.get_load_state(&piece_assets.bn),
        asset_server.get_load_state(&piece_assets.bb),
        asset_server.get_load_state(&piece_assets.bq),
        asset_server.get_load_state(&piece_assets.bk),
        asset_server.get_load_state(&piece_assets.wp),
        asset_server.get_load_state(&piece_assets.wr),
        asset_server.get_load_state(&piece_assets.wn),
        asset_server.get_load_state(&piece_assets.wb),
        asset_server.get_load_state(&piece_assets.wq),
        asset_server.get_load_state(&piece_assets.wk),
    ) {
        state.set(ChessState::Loaded).unwrap();
    }
    
}

pub fn pieces(assets: &PieceAssets, board: &HashMap<&'static str, Vec2>) -> Vec<SpriteBundle> {
    let pz = 0.0;
    vec![
        // black pawns
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("a7").unwrap().x,
                board.get("a7").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("b7").unwrap().x,
                board.get("b7").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("c7").unwrap().x,
                board.get("c7").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("d7").unwrap().x,
                board.get("d7").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("e7").unwrap().x,
                board.get("e7").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("f7").unwrap().x,
                board.get("f7").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("g7").unwrap().x,
                board.get("g7").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("h7").unwrap().x,
                board.get("h7").unwrap().y,
                pz,
            ),
            ..default()
        },
        // black major/minor
        SpriteBundle {
            texture: assets.br.clone(),
            transform: Transform::from_xyz(
                board.get("a8").unwrap().x,
                board.get("a8").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bn.clone(),
            transform: Transform::from_xyz(
                board.get("b8").unwrap().x,
                board.get("b8").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bb.clone(),
            transform: Transform::from_xyz(
                board.get("c8").unwrap().x,
                board.get("c8").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bq.clone(),
            transform: Transform::from_xyz(
                board.get("d8").unwrap().x,
                board.get("d8").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bk.clone(),
            transform: Transform::from_xyz(
                board.get("e8").unwrap().x,
                board.get("e8").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bb.clone(),
            transform: Transform::from_xyz(
                board.get("f8").unwrap().x,
                board.get("f8").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bn.clone(),
            transform: Transform::from_xyz(
                board.get("g8").unwrap().x,
                board.get("g8").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.br.clone(),
            transform: Transform::from_xyz(
                board.get("h8").unwrap().x,
                board.get("h8").unwrap().y,
                pz,
            ),
            ..default()
        },
        // white pawns
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("a2").unwrap().x,
                board.get("a2").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("b2").unwrap().x,
                board.get("b2").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("c2").unwrap().x,
                board.get("c2").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("d2").unwrap().x,
                board.get("d2").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("e2").unwrap().x,
                board.get("e2").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("f2").unwrap().x,
                board.get("f2").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("g2").unwrap().x,
                board.get("g2").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("h2").unwrap().x,
                board.get("h2").unwrap().y,
                pz,
            ),
            ..default()
        },
        // white major/minor
        SpriteBundle {
            texture: assets.wr.clone(),
            transform: Transform::from_xyz(
                board.get("a1").unwrap().x,
                board.get("a1").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wn.clone(),
            transform: Transform::from_xyz(
                board.get("b1").unwrap().x,
                board.get("b1").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wb.clone(),
            transform: Transform::from_xyz(
                board.get("c1").unwrap().x,
                board.get("c1").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wq.clone(),
            transform: Transform::from_xyz(
                board.get("d1").unwrap().x,
                board.get("d1").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wk.clone(),
            transform: Transform::from_xyz(
                board.get("e1").unwrap().x,
                board.get("e1").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wb.clone(),
            transform: Transform::from_xyz(
                board.get("f1").unwrap().x,
                board.get("f1").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wn.clone(),
            transform: Transform::from_xyz(
                board.get("g1").unwrap().x,
                board.get("g1").unwrap().y,
                pz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wr.clone(),
            transform: Transform::from_xyz(
                board.get("h1").unwrap().x,
                board.get("h1").unwrap().y,
                pz,
            ),
            ..default()
        },
    ]
}
