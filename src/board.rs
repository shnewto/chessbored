use bevy::prelude::*;
use std::{collections::HashMap, fmt};

use crate::{assets::BoardAssets, types::Board};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl fmt::Display for File {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            File::A => fmt.write_str("a"),
            File::B => fmt.write_str("b"),
            File::C => fmt.write_str("c"),
            File::D => fmt.write_str("d"),
            File::E => fmt.write_str("e"),
            File::F => fmt.write_str("f"),
            File::G => fmt.write_str("g"),
            File::H => fmt.write_str("h"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Component, Debug, Default)]
pub struct ActiveSquareHighlight {
    pub square: Option<Square>,
    pub stale: bool,
}

#[derive(Component, Debug, Default)]
pub struct LegalMoveHighlight {
    pub stale: bool,
}

impl fmt::Display for Rank {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::One => fmt.write_str("1"),
            Rank::Two => fmt.write_str("2"),
            Rank::Three => fmt.write_str("3"),
            Rank::Four => fmt.write_str("4"),
            Rank::Five => fmt.write_str("5"),
            Rank::Six => fmt.write_str("6"),
            Rank::Seven => fmt.write_str("7"),
            Rank::Eight => fmt.write_str("8"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl fmt::Display for Square {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}{}", self.file, self.rank)
    }
}

impl Square {
    pub fn new(rank: File, file: Rank) -> Square {
        Square {
            file: rank,
            rank: file,
        }
    }

    pub fn location_from_shakmaty(square: shakmaty::Square, board: &Board) -> Option<&Vec2> {
        match square {
            shakmaty::Square::A1 => board.get("a1"),
            shakmaty::Square::A2 => board.get("a2"),
            shakmaty::Square::A3 => board.get("a3"),
            shakmaty::Square::A4 => board.get("a4"),
            shakmaty::Square::A5 => board.get("a5"),
            shakmaty::Square::A6 => board.get("a6"),
            shakmaty::Square::A7 => board.get("a7"),
            shakmaty::Square::A8 => board.get("a8"),

            shakmaty::Square::B1 => board.get("b1"),
            shakmaty::Square::B2 => board.get("b2"),
            shakmaty::Square::B3 => board.get("b3"),
            shakmaty::Square::B4 => board.get("b4"),
            shakmaty::Square::B5 => board.get("b5"),
            shakmaty::Square::B6 => board.get("b6"),
            shakmaty::Square::B7 => board.get("b7"),
            shakmaty::Square::B8 => board.get("b8"),

            shakmaty::Square::C1 => board.get("c1"),
            shakmaty::Square::C2 => board.get("c2"),
            shakmaty::Square::C3 => board.get("c3"),
            shakmaty::Square::C4 => board.get("c4"),
            shakmaty::Square::C5 => board.get("c5"),
            shakmaty::Square::C6 => board.get("c6"),
            shakmaty::Square::C7 => board.get("c7"),
            shakmaty::Square::C8 => board.get("c8"),

            shakmaty::Square::D1 => board.get("d1"),
            shakmaty::Square::D2 => board.get("d2"),
            shakmaty::Square::D3 => board.get("d3"),
            shakmaty::Square::D4 => board.get("d4"),
            shakmaty::Square::D5 => board.get("d5"),
            shakmaty::Square::D6 => board.get("d6"),
            shakmaty::Square::D7 => board.get("d7"),
            shakmaty::Square::D8 => board.get("d8"),

            shakmaty::Square::E1 => board.get("e1"),
            shakmaty::Square::E2 => board.get("e2"),
            shakmaty::Square::E3 => board.get("e3"),
            shakmaty::Square::E4 => board.get("e4"),
            shakmaty::Square::E5 => board.get("e5"),
            shakmaty::Square::E6 => board.get("e6"),
            shakmaty::Square::E7 => board.get("e7"),
            shakmaty::Square::E8 => board.get("e8"),

            shakmaty::Square::F1 => board.get("f1"),
            shakmaty::Square::F2 => board.get("f2"),
            shakmaty::Square::F3 => board.get("f3"),
            shakmaty::Square::F4 => board.get("f4"),
            shakmaty::Square::F5 => board.get("f5"),
            shakmaty::Square::F6 => board.get("f6"),
            shakmaty::Square::F7 => board.get("f7"),
            shakmaty::Square::F8 => board.get("f8"),

            shakmaty::Square::G1 => board.get("g1"),
            shakmaty::Square::G2 => board.get("g2"),
            shakmaty::Square::G3 => board.get("g3"),
            shakmaty::Square::G4 => board.get("g4"),
            shakmaty::Square::G5 => board.get("g5"),
            shakmaty::Square::G6 => board.get("g6"),
            shakmaty::Square::G7 => board.get("g7"),
            shakmaty::Square::G8 => board.get("g8"),

            shakmaty::Square::H1 => board.get("h1"),
            shakmaty::Square::H2 => board.get("h2"),
            shakmaty::Square::H3 => board.get("h3"),
            shakmaty::Square::H4 => board.get("h4"),
            shakmaty::Square::H5 => board.get("h5"),
            shakmaty::Square::H6 => board.get("h6"),
            shakmaty::Square::H7 => board.get("h7"),
            shakmaty::Square::H8 => board.get("h8"),
        }
    }

    pub fn to_shakmaty(square: Square) -> shakmaty::Square {
        match (square.file, square.rank) {
            (File::A, Rank::One) => shakmaty::Square::A1,
            (File::A, Rank::Two) => shakmaty::Square::A2,
            (File::A, Rank::Three) => shakmaty::Square::A3,
            (File::A, Rank::Four) => shakmaty::Square::A4,
            (File::A, Rank::Five) => shakmaty::Square::A5,
            (File::A, Rank::Six) => shakmaty::Square::A6,
            (File::A, Rank::Seven) => shakmaty::Square::A7,
            (File::A, Rank::Eight) => shakmaty::Square::A8,

            (File::B, Rank::One) => shakmaty::Square::B1,
            (File::B, Rank::Two) => shakmaty::Square::B2,
            (File::B, Rank::Three) => shakmaty::Square::B3,
            (File::B, Rank::Four) => shakmaty::Square::B4,
            (File::B, Rank::Five) => shakmaty::Square::B5,
            (File::B, Rank::Six) => shakmaty::Square::B6,
            (File::B, Rank::Seven) => shakmaty::Square::B7,
            (File::B, Rank::Eight) => shakmaty::Square::B8,

            (File::C, Rank::One) => shakmaty::Square::C1,
            (File::C, Rank::Two) => shakmaty::Square::C2,
            (File::C, Rank::Three) => shakmaty::Square::C3,
            (File::C, Rank::Four) => shakmaty::Square::C4,
            (File::C, Rank::Five) => shakmaty::Square::C5,
            (File::C, Rank::Six) => shakmaty::Square::C6,
            (File::C, Rank::Seven) => shakmaty::Square::C7,
            (File::C, Rank::Eight) => shakmaty::Square::C8,

            (File::D, Rank::One) => shakmaty::Square::D1,
            (File::D, Rank::Two) => shakmaty::Square::D2,
            (File::D, Rank::Three) => shakmaty::Square::D3,
            (File::D, Rank::Four) => shakmaty::Square::D4,
            (File::D, Rank::Five) => shakmaty::Square::D5,
            (File::D, Rank::Six) => shakmaty::Square::D6,
            (File::D, Rank::Seven) => shakmaty::Square::D7,
            (File::D, Rank::Eight) => shakmaty::Square::D8,

            (File::E, Rank::One) => shakmaty::Square::E1,
            (File::E, Rank::Two) => shakmaty::Square::E2,
            (File::E, Rank::Three) => shakmaty::Square::E3,
            (File::E, Rank::Four) => shakmaty::Square::E4,
            (File::E, Rank::Five) => shakmaty::Square::E5,
            (File::E, Rank::Six) => shakmaty::Square::E6,
            (File::E, Rank::Seven) => shakmaty::Square::E7,
            (File::E, Rank::Eight) => shakmaty::Square::E8,

            (File::F, Rank::One) => shakmaty::Square::F1,
            (File::F, Rank::Two) => shakmaty::Square::F2,
            (File::F, Rank::Three) => shakmaty::Square::F3,
            (File::F, Rank::Four) => shakmaty::Square::F4,
            (File::F, Rank::Five) => shakmaty::Square::F5,
            (File::F, Rank::Six) => shakmaty::Square::F6,
            (File::F, Rank::Seven) => shakmaty::Square::F7,
            (File::F, Rank::Eight) => shakmaty::Square::F8,

            (File::G, Rank::One) => shakmaty::Square::G1,
            (File::G, Rank::Two) => shakmaty::Square::G2,
            (File::G, Rank::Three) => shakmaty::Square::G3,
            (File::G, Rank::Four) => shakmaty::Square::G4,
            (File::G, Rank::Five) => shakmaty::Square::G5,
            (File::G, Rank::Six) => shakmaty::Square::G6,
            (File::G, Rank::Seven) => shakmaty::Square::G7,
            (File::G, Rank::Eight) => shakmaty::Square::G8,

            (File::H, Rank::One) => shakmaty::Square::H1,
            (File::H, Rank::Two) => shakmaty::Square::H2,
            (File::H, Rank::Three) => shakmaty::Square::H3,
            (File::H, Rank::Four) => shakmaty::Square::H4,
            (File::H, Rank::Five) => shakmaty::Square::H5,
            (File::H, Rank::Six) => shakmaty::Square::H6,
            (File::H, Rank::Seven) => shakmaty::Square::H7,
            (File::H, Rank::Eight) => shakmaty::Square::H8,
        }
    }
}

pub fn get_square(x: f32, y: f32) -> Option<Square> {
    // on the board
    if y >= -25.0 && x >= -25.0 && y <= 365.0 && x <= 365.0 {
        let rank = if x <= 25.0 {
            File::A
        } else if x <= 75.0 {
            File::B
        } else if x <= 125.0 {
            File::C
        } else if x <= 175.0 {
            File::D
        } else if x <= 225.0 {
            File::E
        } else if x <= 275.0 {
            File::F
        } else if x <= 325.0 {
            File::G
        } else {
            File::H
        };

        let file = if y <= 25.0 {
            Rank::One
        } else if y <= 75.0 {
            Rank::Two
        } else if y <= 125.0 {
            Rank::Three
        } else if y <= 175.0 {
            Rank::Four
        } else if y <= 225.0 {
            Rank::Five
        } else if y <= 275.0 {
            Rank::Six
        } else if y <= 325.0 {
            Rank::Seven
        } else {
            Rank::Eight
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
