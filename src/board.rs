use bevy::prelude::*;
use std::{collections::HashMap, fmt};

use crate::{assets::BoardAssets, types::Board};

#[derive(Debug, Clone, PartialEq)]
pub enum Rank {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl fmt::Display for Rank {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::A => fmt.write_str("a"),
            Rank::B => fmt.write_str("b"),
            Rank::C => fmt.write_str("c"),
            Rank::D => fmt.write_str("d"),
            Rank::E => fmt.write_str("e"),
            Rank::F => fmt.write_str("f"),
            Rank::G => fmt.write_str("g"),
            Rank::H => fmt.write_str("h"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum File {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl fmt::Display for File {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            File::One => fmt.write_str("1"),
            File::Two => fmt.write_str("2"),
            File::Three => fmt.write_str("3"),
            File::Four => fmt.write_str("4"),
            File::Five => fmt.write_str("5"),
            File::Six => fmt.write_str("6"),
            File::Seven => fmt.write_str("7"),
            File::Eight => fmt.write_str("8"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Square {
    pub rank: Rank,
    pub file: File,
}

impl fmt::Display for Square {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}{}", self.rank, self.file)
    }
}

impl Square {
    pub fn new(rank: Rank, file: File) -> Square {
        Square { rank, file }
    }
}

pub fn get_square(x: f32, y: f32) -> Option<Square> {
    // on the board
    if y >= -25.0 && x >= -25.0 && y <= 365.0 && x <= 365.0 {
        let rank = if x <= 15.0 {
            Rank::A
        } else if x <= 65.0 {
            Rank::B
        } else if x <= 115.0 {
            Rank::C
        } else if x <= 165.0 {
            Rank::D
        } else if x <= 215.0 {
            Rank::E
        } else if x <= 265.0 {
            Rank::F
        } else if x <= 315.0 {
            Rank::G
        } else {
            Rank::H
        };

        let file = if y <= 15.0 {
            File::One
        } else if y <= 65.0 {
            File::Two
        } else if y <= 115.0 {
            File::Three
        } else if y <= 165.0 {
            File::Four
        } else if y <= 215.0 {
            File::Five
        } else if y <= 265.0 {
            File::Six
        } else if y <= 315.0 {
            File::Seven
        } else {
            File::Eight
        };

        Some(Square::new(rank, file))
    } else {
        None
    }
}

pub fn board_map() -> Board {
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

    // side selection
    board.insert("wq", Vec2::new(410.0, 100.0));
    board.insert("wk", Vec2::new(460.0, 100.0));
    board.insert("wn", Vec2::new(410.0, 50.0));
    board.insert("wb", Vec2::new(460.0, 50.0));
    board.insert("wp", Vec2::new(410.0, 0.0));
    board.insert("wr", Vec2::new(460.0, 0.0));

    board.insert("bq", Vec2::new(410.0, 350.0));
    board.insert("bk", Vec2::new(460.0, 350.0));
    board.insert("bn", Vec2::new(410.0, 300.0));
    board.insert("bb", Vec2::new(460.0, 300.0));
    board.insert("bp", Vec2::new(410.0, 250.0));
    board.insert("br", Vec2::new(460.0, 250.0));

    board
}

pub fn setup_board(mut commands: Commands, assets: ResMut<BoardAssets>, board: Res<Board>) {
    let sz = -0.01;

    // row 1
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("a1").unwrap().x, board.get("a1").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("b1").unwrap().x, board.get("b1").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("c1").unwrap().x, board.get("c1").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("d1").unwrap().x, board.get("d1").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("e1").unwrap().x, board.get("e1").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("f1").unwrap().x, board.get("f1").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("g1").unwrap().x, board.get("g1").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("h1").unwrap().x, board.get("h1").unwrap().y, sz),
        ..default()
    });
    // row 2
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("a2").unwrap().x, board.get("a2").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("b2").unwrap().x, board.get("b2").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("c2").unwrap().x, board.get("c2").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("d2").unwrap().x, board.get("d2").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("e2").unwrap().x, board.get("e2").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("f2").unwrap().x, board.get("f2").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("g2").unwrap().x, board.get("g2").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("h2").unwrap().x, board.get("h2").unwrap().y, sz),
        ..default()
    });
    // row 3
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("a3").unwrap().x, board.get("a3").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("b3").unwrap().x, board.get("b3").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("c3").unwrap().x, board.get("c3").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("d3").unwrap().x, board.get("d3").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("e3").unwrap().x, board.get("e3").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("f3").unwrap().x, board.get("f3").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("g3").unwrap().x, board.get("g3").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("h3").unwrap().x, board.get("h3").unwrap().y, sz),
        ..default()
    });
    // row 4
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("a4").unwrap().x, board.get("a4").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("b4").unwrap().x, board.get("b4").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("c4").unwrap().x, board.get("c4").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("d4").unwrap().x, board.get("d4").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("e4").unwrap().x, board.get("e4").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("f4").unwrap().x, board.get("f4").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("g4").unwrap().x, board.get("g4").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("h4").unwrap().x, board.get("h4").unwrap().y, sz),
        ..default()
    });
    // row 5
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("a5").unwrap().x, board.get("a5").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("b5").unwrap().x, board.get("b5").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("c5").unwrap().x, board.get("c5").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("d5").unwrap().x, board.get("d5").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("e5").unwrap().x, board.get("e5").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("f5").unwrap().x, board.get("f5").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("g5").unwrap().x, board.get("g5").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("h5").unwrap().x, board.get("h5").unwrap().y, sz),
        ..default()
    });
    // row 6
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("a6").unwrap().x, board.get("a6").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("b6").unwrap().x, board.get("b6").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("c6").unwrap().x, board.get("c6").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("d6").unwrap().x, board.get("d6").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("e6").unwrap().x, board.get("e6").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("f6").unwrap().x, board.get("f6").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("g6").unwrap().x, board.get("g6").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("h6").unwrap().x, board.get("h6").unwrap().y, sz),
        ..default()
    });
    // row 7
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("a7").unwrap().x, board.get("a7").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("b7").unwrap().x, board.get("b7").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("c7").unwrap().x, board.get("c7").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("d7").unwrap().x, board.get("d7").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("e7").unwrap().x, board.get("e7").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("f7").unwrap().x, board.get("f7").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("g7").unwrap().x, board.get("g7").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("h7").unwrap().x, board.get("h7").unwrap().y, sz),
        ..default()
    });
    // row 8
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("a8").unwrap().x, board.get("a8").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("b8").unwrap().x, board.get("b8").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("c8").unwrap().x, board.get("c8").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("d8").unwrap().x, board.get("d8").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("e8").unwrap().x, board.get("e8").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("f8").unwrap().x, board.get("f8").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.light_square_handle.clone(),
        transform: Transform::from_xyz(board.get("g8").unwrap().x, board.get("g8").unwrap().y, sz),
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.dark_square_handle.clone(),
        transform: Transform::from_xyz(board.get("h8").unwrap().x, board.get("h8").unwrap().y, sz),
        ..default()
    });
}
