use bevy::{asset::LoadState, prelude::*};
use std::collections::HashMap;

type Board = HashMap<&'static str, Vec2>;

fn board_map() -> Board {
    let mut board = HashMap::new();

    board.insert("a1", Vec2::new(0.0, 0.0));
    board.insert("b1", Vec2::new(50.0, 0.0));
    board.insert("c1", Vec2::new(100.0, 0.0));
    board.insert("d1", Vec2::new(150.0, 0.0));
    board.insert("e1", Vec2::new(200.0, 0.0));
    board.insert("f1", Vec2::new(250.0, 0.0));
    board.insert("g1", Vec2::new(300.0, 0.0));
    board.insert("h1", Vec2::new(350.0, 0.0));

    board.insert("a2", Vec2::new(0.0, 50.0));
    board.insert("b2", Vec2::new(50.0, 50.0));
    board.insert("c2", Vec2::new(100.0, 50.0));
    board.insert("d2", Vec2::new(150.0, 50.0));
    board.insert("e2", Vec2::new(200.0, 50.0));
    board.insert("f2", Vec2::new(250.0, 50.0));
    board.insert("g2", Vec2::new(300.0, 50.0));
    board.insert("h2", Vec2::new(350.0, 50.0));

    board.insert("a3", Vec2::new(0.0, 100.0));
    board.insert("b3", Vec2::new(50.0, 100.0));
    board.insert("c3", Vec2::new(100.0, 100.0));
    board.insert("d3", Vec2::new(150.0, 100.0));
    board.insert("e3", Vec2::new(200.0, 100.0));
    board.insert("f3", Vec2::new(250.0, 100.0));
    board.insert("g3", Vec2::new(300.0, 100.0));
    board.insert("h3", Vec2::new(350.0, 100.0));

    board.insert("a4", Vec2::new(0.0, 150.0));
    board.insert("b4", Vec2::new(50.0, 150.0));
    board.insert("c4", Vec2::new(100.0, 150.0));
    board.insert("d4", Vec2::new(150.0, 150.0));
    board.insert("e4", Vec2::new(200.0, 150.0));
    board.insert("f4", Vec2::new(250.0, 150.0));
    board.insert("g4", Vec2::new(300.0, 150.0));
    board.insert("h4", Vec2::new(350.0, 150.0));

    board.insert("a5", Vec2::new(0.0, 200.0));
    board.insert("b5", Vec2::new(50.0, 200.0));
    board.insert("c5", Vec2::new(100.0, 200.0));
    board.insert("d5", Vec2::new(150.0, 200.0));
    board.insert("e5", Vec2::new(200.0, 200.0));
    board.insert("f5", Vec2::new(250.0, 200.0));
    board.insert("g5", Vec2::new(300.0, 200.0));
    board.insert("h5", Vec2::new(350.0, 200.0));

    board.insert("a6", Vec2::new(0.0, 250.0));
    board.insert("b6", Vec2::new(50.0, 250.0));
    board.insert("c6", Vec2::new(100.0, 250.0));
    board.insert("d6", Vec2::new(150.0, 250.0));
    board.insert("e6", Vec2::new(200.0, 250.0));
    board.insert("f6", Vec2::new(250.0, 250.0));
    board.insert("g6", Vec2::new(300.0, 250.0));
    board.insert("h6", Vec2::new(350.0, 250.0));

    board.insert("a7", Vec2::new(0.0, 300.0));
    board.insert("b7", Vec2::new(50.0, 300.0));
    board.insert("c7", Vec2::new(100.0, 300.0));
    board.insert("d7", Vec2::new(150.0, 300.0));
    board.insert("e7", Vec2::new(200.0, 300.0));
    board.insert("f7", Vec2::new(250.0, 300.0));
    board.insert("g7", Vec2::new(300.0, 300.0));
    board.insert("h7", Vec2::new(350.0, 300.0));

    board.insert("a8", Vec2::new(0.0, 350.0));
    board.insert("b8", Vec2::new(50.0, 350.0));
    board.insert("c8", Vec2::new(100.0, 350.0));
    board.insert("d8", Vec2::new(150.0, 350.0));
    board.insert("e8", Vec2::new(200.0, 350.0));
    board.insert("f8", Vec2::new(250.0, 350.0));
    board.insert("g8", Vec2::new(300.0, 350.0));
    board.insert("h8", Vec2::new(350.0, 350.0));

    board
}

fn squares(assets: &BoardAssets, board: &HashMap<&'static str, Vec2>) -> Vec<SpriteBundle> {
    let sz = -0.1;
    vec![
        // row 1
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a1").unwrap().x,
                board.get("a1").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b1").unwrap().x,
                board.get("b1").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c1").unwrap().x,
                board.get("c1").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d1").unwrap().x,
                board.get("d1").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e1").unwrap().x,
                board.get("e1").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f1").unwrap().x,
                board.get("f1").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g1").unwrap().x,
                board.get("g1").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h1").unwrap().x,
                board.get("h1").unwrap().y,
                sz,
            ),
            ..default()
        },
        // row 2
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a2").unwrap().x,
                board.get("a2").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b2").unwrap().x,
                board.get("b2").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c2").unwrap().x,
                board.get("c2").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d2").unwrap().x,
                board.get("d2").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e2").unwrap().x,
                board.get("e2").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f2").unwrap().x,
                board.get("f2").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g2").unwrap().x,
                board.get("g2").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h2").unwrap().x,
                board.get("h2").unwrap().y,
                sz,
            ),
            ..default()
        },
        // row 3
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a3").unwrap().x,
                board.get("a3").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b3").unwrap().x,
                board.get("b3").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c3").unwrap().x,
                board.get("c3").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d3").unwrap().x,
                board.get("d3").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e3").unwrap().x,
                board.get("e3").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f3").unwrap().x,
                board.get("f3").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g3").unwrap().x,
                board.get("g3").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h3").unwrap().x,
                board.get("h3").unwrap().y,
                sz,
            ),
            ..default()
        },
        // row 4
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a4").unwrap().x,
                board.get("a4").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b4").unwrap().x,
                board.get("b4").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c4").unwrap().x,
                board.get("c4").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d4").unwrap().x,
                board.get("d4").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e4").unwrap().x,
                board.get("e4").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f4").unwrap().x,
                board.get("f4").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g4").unwrap().x,
                board.get("g4").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h4").unwrap().x,
                board.get("h4").unwrap().y,
                sz,
            ),
            ..default()
        },
        // row 5
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a5").unwrap().x,
                board.get("a5").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b5").unwrap().x,
                board.get("b5").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c5").unwrap().x,
                board.get("c5").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d5").unwrap().x,
                board.get("d5").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e5").unwrap().x,
                board.get("e5").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f5").unwrap().x,
                board.get("f5").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g5").unwrap().x,
                board.get("g5").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h5").unwrap().x,
                board.get("h5").unwrap().y,
                sz,
            ),
            ..default()
        },
        // row 6
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a6").unwrap().x,
                board.get("a6").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b6").unwrap().x,
                board.get("b6").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c6").unwrap().x,
                board.get("c6").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d6").unwrap().x,
                board.get("d6").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e6").unwrap().x,
                board.get("e6").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f6").unwrap().x,
                board.get("f6").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g6").unwrap().x,
                board.get("g6").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h6").unwrap().x,
                board.get("h6").unwrap().y,
                sz,
            ),
            ..default()
        },
        // row 7
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a7").unwrap().x,
                board.get("a7").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b7").unwrap().x,
                board.get("b7").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c7").unwrap().x,
                board.get("c7").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d7").unwrap().x,
                board.get("d7").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e7").unwrap().x,
                board.get("e7").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f7").unwrap().x,
                board.get("f7").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g7").unwrap().x,
                board.get("g7").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h7").unwrap().x,
                board.get("h7").unwrap().y,
                sz,
            ),
            ..default()
        },
        // row 8
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a8").unwrap().x,
                board.get("a8").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b8").unwrap().x,
                board.get("b8").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c8").unwrap().x,
                board.get("c8").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d8").unwrap().x,
                board.get("d8").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e8").unwrap().x,
                board.get("e8").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f8").unwrap().x,
                board.get("f8").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g8").unwrap().x,
                board.get("g8").unwrap().y,
                sz,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h8").unwrap().x,
                board.get("h8").unwrap().y,
                sz,
            ),
            ..default()
        },
    ]
}

fn pieces(assets: &BoardAssets, board: &HashMap<&'static str, Vec2>) -> Vec<SpriteBundle> {
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

pub fn load_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut board_assets: ResMut<BoardAssets>,
    board: Res<Board>,
) {
    board_assets.dark_square_handle = asset_server.load("board/dark-square.png");
    board_assets.light_square_handle = asset_server.load("board/light-square.png");
    commands.spawn_batch(squares(&board_assets, &board));
}

pub fn load_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut board_assets: ResMut<BoardAssets>,
    board: Res<Board>,
) {
    board_assets.bp = asset_server.load("pieces/bp.png");
    board_assets.br = asset_server.load("pieces/br.png");
    board_assets.bn = asset_server.load("pieces/bn.png");
    board_assets.bb = asset_server.load("pieces/bq.png");
    board_assets.bq = asset_server.load("pieces/bb.png");
    board_assets.bk = asset_server.load("pieces/bk.png");

    board_assets.wp = asset_server.load("pieces/wp.png");
    board_assets.wr = asset_server.load("pieces/wr.png");
    board_assets.wn = asset_server.load("pieces/wn.png");
    board_assets.wb = asset_server.load("pieces/wb.png");
    board_assets.wq = asset_server.load("pieces/wq.png");
    board_assets.wk = asset_server.load("pieces/wk.png");

    commands.spawn_batch(pieces(&board_assets, &board));
}

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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ChessState {
    Setup,
    LoadingBoard,
    LoadingPieces,
    Loaded,
}

pub fn main() {
    App::new()
        .insert_resource(BoardAssets::default())
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

fn check_board(
    mut state: ResMut<State<ChessState>>,
    asset_server: Res<AssetServer>,
    board_assets: Res<BoardAssets>,
) {
    if let (LoadState::Loaded,) = (asset_server.get_load_state(&board_assets.dark_square_handle),) {
        println!("dark_square_handle loaded")
    } else {
        return;
    }

    if let (LoadState::Loaded,) = (asset_server.get_load_state(&board_assets.light_square_handle),)
    {
        println!("light_square_handle loaded")
    } else {
        return;
    }
    state.set(ChessState::LoadingPieces).unwrap();
}

fn check_pieces(
    mut state: ResMut<State<ChessState>>,
    asset_server: Res<AssetServer>,
    board_assets: Res<BoardAssets>,
) {
    if let (LoadState::Loaded,) = (asset_server.get_load_state(&board_assets.bp),) {
        println!("bp loaded")
    } else {
        return;
    }

    if let (LoadState::Loaded,) = (asset_server.get_load_state(&board_assets.br),) {
        println!("br loaded")
    } else {
        return;
    }

    if let (LoadState::Loaded,) = (asset_server.get_load_state(&board_assets.bn),) {
        println!("bn loaded")
    } else {
        return;
    }

    if let (LoadState::Loaded,) = (asset_server.get_load_state(&board_assets.bb),) {
        println!("bb loaded")
    } else {
        return;
    }

    if let (LoadState::Loaded,) = (asset_server.get_load_state(&board_assets.bq),) {
        println!("bq loaded")
    } else {
        return;
    }

    if let (LoadState::Loaded,) = (asset_server.get_load_state(&board_assets.bk),) {
        println!("bk loaded")
    } else {
        return;
    }

    if let (LoadState::Loaded,) = (asset_server.get_load_state(&board_assets.wp),) {
        println!("wp loaded")
    } else {
        return;
    }

    if let (LoadState::Loaded,) = (asset_server.get_load_state(&board_assets.wr),) {
        println!("wr loaded")
    } else {
        return;
    }

    if let (LoadState::Loaded,) = (asset_server.get_load_state(&board_assets.wn),) {
        println!("wn loaded")
    } else {
        return;
    }

    if let (LoadState::Loaded,) = (asset_server.get_load_state(&board_assets.wb),) {
        println!("wb loaded")
    } else {
        return;
    }

    if let (LoadState::Loaded,) = (asset_server.get_load_state(&board_assets.wq),) {
        println!("wq loaded")
    } else {
        return;
    }

    if let (LoadState::Loaded,) = (asset_server.get_load_state(&board_assets.wk),) {
        println!("wk loaded")
    } else {
        return;
    }

    println!("all loaded!");
    state.set(ChessState::Loaded).unwrap();
}
