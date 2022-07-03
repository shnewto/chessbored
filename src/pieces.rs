use bevy::{asset::LoadState, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::*;

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

pub fn load_piece_assets(asset_server: Res<AssetServer>, mut piece_assets: ResMut<PieceAssets>) {
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

pub fn spawn_pieces(
    mut commands: Commands,
    assets: ResMut<PieceAssets>,
    board: Res<Board>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let pz = 0.0;

    // black pawns
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("a7").unwrap().x,
                board.get("a7").unwrap().y,
                pz,
            ),
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("b7").unwrap().x,
                board.get("b7").unwrap().y,
                pz,
            ),
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("c7").unwrap().x,
                board.get("c7").unwrap().y,
                pz,
            ),
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("d7").unwrap().x,
                board.get("d7").unwrap().y,
                pz,
            ),
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("e7").unwrap().x,
                board.get("e7").unwrap().y,
                pz,
            ),
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("f7").unwrap().x,
                board.get("f7").unwrap().y,
                pz,
            ),
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("g7").unwrap().x,
                board.get("g7").unwrap().y,
                pz,
            ),
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("h7").unwrap().x,
                board.get("h7").unwrap().y,
                pz,
            ),
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    // black major/minor
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("a8").unwrap().x,
                board.get("a8").unwrap().y,
                pz,
            ),
            material: materials.add(assets.br.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("b8").unwrap().x,
                board.get("b8").unwrap().y,
                pz,
            ),
            material: materials.add(assets.bn.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("c8").unwrap().x,
                board.get("c8").unwrap().y,
                pz,
            ),
            material: materials.add(assets.bb.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("d8").unwrap().x,
                board.get("d8").unwrap().y,
                pz,
            ),
            material: materials.add(assets.bq.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("e8").unwrap().x,
                board.get("e8").unwrap().y,
                pz,
            ),
            material: materials.add(assets.bk.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("f8").unwrap().x,
                board.get("f8").unwrap().y,
                pz,
            ),
            material: materials.add(assets.bb.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("g8").unwrap().x,
                board.get("g8").unwrap().y,
                pz,
            ),
            material: materials.add(assets.bn.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("h8").unwrap().x,
                board.get("h8").unwrap().y,
                pz,
            ),
            material: materials.add(assets.br.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    // white pawns
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("a2").unwrap().x,
                board.get("a2").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("b2").unwrap().x,
                board.get("b2").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("c2").unwrap().x,
                board.get("c2").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("d2").unwrap().x,
                board.get("d2").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("e2").unwrap().x,
                board.get("e2").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("f2").unwrap().x,
                board.get("f2").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("g2").unwrap().x,
                board.get("g2").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("h2").unwrap().x,
                board.get("h2").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    // white major/minor
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("a1").unwrap().x,
                board.get("a1").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wr.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("b1").unwrap().x,
                board.get("b1").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wn.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("c1").unwrap().x,
                board.get("c1").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wb.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("d1").unwrap().x,
                board.get("d1").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wq.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("e1").unwrap().x,
                board.get("e1").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wk.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("f1").unwrap().x,
                board.get("f1").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wb.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("g1").unwrap().x,
                board.get("g1").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wn.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2::new(50.0, 50.0), ..default()})).into(),
            transform: Transform::from_xyz(
                board.get("h1").unwrap().x,
                board.get("h1").unwrap().y,
                pz,
            ),
            material: materials.add(assets.wr.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
}
