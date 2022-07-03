use bevy::{asset::LoadState, prelude::*};
use std::collections::HashMap;

fn board_map() -> HashMap<&'static str, Vec2> {
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
    vec![
        // row 1
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a1").unwrap().x,
                board.get("a1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b1").unwrap().x,
                board.get("b1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c1").unwrap().x,
                board.get("c1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d1").unwrap().x,
                board.get("d1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e1").unwrap().x,
                board.get("e1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f1").unwrap().x,
                board.get("f1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g1").unwrap().x,
                board.get("g1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h1").unwrap().x,
                board.get("h1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        // row 2
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a2").unwrap().x,
                board.get("a2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b2").unwrap().x,
                board.get("b2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c2").unwrap().x,
                board.get("c2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d2").unwrap().x,
                board.get("d2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e2").unwrap().x,
                board.get("e2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f2").unwrap().x,
                board.get("f2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g2").unwrap().x,
                board.get("g2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h2").unwrap().x,
                board.get("h2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        // row 3
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a3").unwrap().x,
                board.get("a3").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b3").unwrap().x,
                board.get("b3").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c3").unwrap().x,
                board.get("c3").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d3").unwrap().x,
                board.get("d3").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e3").unwrap().x,
                board.get("e3").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f3").unwrap().x,
                board.get("f3").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g3").unwrap().x,
                board.get("g3").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h3").unwrap().x,
                board.get("h3").unwrap().y,
                0.0,
            ),
            ..default()
        },
        // row 4
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a4").unwrap().x,
                board.get("a4").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b4").unwrap().x,
                board.get("b4").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c4").unwrap().x,
                board.get("c4").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d4").unwrap().x,
                board.get("d4").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e4").unwrap().x,
                board.get("e4").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f4").unwrap().x,
                board.get("f4").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g4").unwrap().x,
                board.get("g4").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h4").unwrap().x,
                board.get("h4").unwrap().y,
                0.0,
            ),
            ..default()
        },
        // row 5
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a5").unwrap().x,
                board.get("a5").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b5").unwrap().x,
                board.get("b5").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c5").unwrap().x,
                board.get("c5").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d5").unwrap().x,
                board.get("d5").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e5").unwrap().x,
                board.get("e5").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f5").unwrap().x,
                board.get("f5").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g5").unwrap().x,
                board.get("g5").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h5").unwrap().x,
                board.get("h5").unwrap().y,
                0.0,
            ),
            ..default()
        },
        // row 6
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a6").unwrap().x,
                board.get("a6").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b6").unwrap().x,
                board.get("b6").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c6").unwrap().x,
                board.get("c6").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d6").unwrap().x,
                board.get("d6").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e6").unwrap().x,
                board.get("e6").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f6").unwrap().x,
                board.get("f6").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g6").unwrap().x,
                board.get("g6").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h6").unwrap().x,
                board.get("h6").unwrap().y,
                0.0,
            ),
            ..default()
        },
        // row 7
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a7").unwrap().x,
                board.get("a7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b7").unwrap().x,
                board.get("b7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c7").unwrap().x,
                board.get("c7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d7").unwrap().x,
                board.get("d7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e7").unwrap().x,
                board.get("e7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f7").unwrap().x,
                board.get("f7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g7").unwrap().x,
                board.get("g7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h7").unwrap().x,
                board.get("h7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        // row 8
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("a8").unwrap().x,
                board.get("a8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("b8").unwrap().x,
                board.get("b8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("c8").unwrap().x,
                board.get("c8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("d8").unwrap().x,
                board.get("d8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("e8").unwrap().x,
                board.get("e8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("f8").unwrap().x,
                board.get("f8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.light_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("g8").unwrap().x,
                board.get("g8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.dark_square_handle.clone(),
            transform: Transform::from_xyz(
                board.get("h8").unwrap().x,
                board.get("h8").unwrap().y,
                0.0,
            ),
            ..default()
        },
    ]
}

fn pieces(assets: &BoardAssets, board: &HashMap<&'static str, Vec2>) -> Vec<SpriteBundle> {
    vec![
        // black pawns
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("a7").unwrap().x,
                board.get("a7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("b7").unwrap().x,
                board.get("b7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("c7").unwrap().x,
                board.get("c7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("d7").unwrap().x,
                board.get("d7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("e7").unwrap().x,
                board.get("e7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("f7").unwrap().x,
                board.get("f7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("g7").unwrap().x,
                board.get("g7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bp.clone(),
            transform: Transform::from_xyz(
                board.get("h7").unwrap().x,
                board.get("h7").unwrap().y,
                0.0,
            ),
            ..default()
        },
        // black major/minor
        SpriteBundle {
            texture: assets.br.clone(),
            transform: Transform::from_xyz(
                board.get("a8").unwrap().x,
                board.get("a8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bn.clone(),
            transform: Transform::from_xyz(
                board.get("b8").unwrap().x,
                board.get("b8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bb.clone(),
            transform: Transform::from_xyz(
                board.get("c8").unwrap().x,
                board.get("c8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bq.clone(),
            transform: Transform::from_xyz(
                board.get("d8").unwrap().x,
                board.get("d8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bk.clone(),
            transform: Transform::from_xyz(
                board.get("e8").unwrap().x,
                board.get("e8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bb.clone(),
            transform: Transform::from_xyz(
                board.get("f8").unwrap().x,
                board.get("f8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.bn.clone(),
            transform: Transform::from_xyz(
                board.get("g8").unwrap().x,
                board.get("g8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.br.clone(),
            transform: Transform::from_xyz(
                board.get("h8").unwrap().x,
                board.get("h8").unwrap().y,
                0.0,
            ),
            ..default()
        },
        // white pawns
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("a2").unwrap().x,
                board.get("a2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("b2").unwrap().x,
                board.get("b2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("c2").unwrap().x,
                board.get("c2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("d2").unwrap().x,
                board.get("d2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("e2").unwrap().x,
                board.get("e2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("f2").unwrap().x,
                board.get("f2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("g2").unwrap().x,
                board.get("g2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wp.clone(),
            transform: Transform::from_xyz(
                board.get("h2").unwrap().x,
                board.get("h2").unwrap().y,
                0.0,
            ),
            ..default()
        },
        // white major/minor
        SpriteBundle {
            texture: assets.wr.clone(),
            transform: Transform::from_xyz(
                board.get("a1").unwrap().x,
                board.get("a1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wn.clone(),
            transform: Transform::from_xyz(
                board.get("b1").unwrap().x,
                board.get("b1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wb.clone(),
            transform: Transform::from_xyz(
                board.get("c1").unwrap().x,
                board.get("c1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wq.clone(),
            transform: Transform::from_xyz(
                board.get("d1").unwrap().x,
                board.get("d1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wk.clone(),
            transform: Transform::from_xyz(
                board.get("e1").unwrap().x,
                board.get("e1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wb.clone(),
            transform: Transform::from_xyz(
                board.get("f1").unwrap().x,
                board.get("f1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wn.clone(),
            transform: Transform::from_xyz(
                board.get("g1").unwrap().x,
                board.get("g1").unwrap().y,
                0.0,
            ),
            ..default()
        },
        SpriteBundle {
            texture: assets.wr.clone(),
            transform: Transform::from_xyz(
                board.get("h1").unwrap().x,
                board.get("h1").unwrap().y,
                0.0,
            ),
            ..default()
        },
    ]
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut board_assets: ResMut<BoardAssets>,
) {
    let board = board_map();
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(200.0, 200.0, 0.0),
        ..default()
    });

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

    commands.spawn_batch(squares(&board_assets, &board));
    commands.spawn_batch(pieces(&board_assets, &board));
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ChessState {
    LoadingAssets,
    Loaded,
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

pub fn main() {
    App::new()
        .insert_resource(BoardAssets::default())
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(ChessState::LoadingAssets).with_system(setup))
        .add_system_set(SystemSet::on_update(ChessState::LoadingAssets).with_system(check))
        .add_system_set(SystemSet::on_enter(ChessState::Loaded).with_system(loaded))
        .run();
}

fn loaded() {}

fn check(
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
