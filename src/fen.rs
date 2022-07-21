use bevy::prelude::*;

use crate::{
    assets::FenAssets,
    board::{get_square, File, Rank, Square},
    pieces::{ActivePiece, Piece, SelectedPiece, SourcePiece},
};

#[derive(Component)]
pub struct FenElement;

#[derive(Component)]
pub struct FenText;

pub fn spawn(mut commands: Commands, fen_assets: Res<FenAssets>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(400.0), Val::Px(20.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: Rect {
                    left: Val::Px(160.0),
                    bottom: Val::Px(80.0),
                    ..default()
                },
                ..default()
            },
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..Default::default()
        })
        .insert(FenElement)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "FEN NOTATION".to_string(),
                            style: TextStyle {
                                font: fen_assets.fen_font_handle.clone(),
                                font_size: 12.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        }],
                        alignment: TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    },
                    ..Default::default()
                })
                .insert(FenText);
        });

    let clear_color_hex_string = "69696b";

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(400.0), Val::Px(10.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: Rect {
                    left: Val::Px(160.0),
                    bottom: Val::Px(70.0),
                    ..default()
                },
                ..default()
            },
            color: Color::hex(clear_color_hex_string)
                .unwrap_or_else(|_| {
                    panic!("couldn't make hex color from {}", clear_color_hex_string)
                })
                .into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "(click to copy)".to_string(),
                        style: TextStyle {
                            font: fen_assets.tool_tip_font_handle.clone(),
                            font_size: 10.0,
                            color: Color::rgb(0.15, 0.15, 0.15),
                        },
                    }],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                },
                ..Default::default()
            });
        });
}

type ButtonInteraction<'a> = (&'a Interaction, &'a Children);

pub fn copy_to_clipboard(
    mut clipboard: ResMut<bevy_egui::EguiClipboard>,
    interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<FenElement>)>,
    text_query: Query<&Text>,
) {
    for (interaction, children) in interaction_query.iter() {
        let text = text_query.get(children[0]).unwrap();

        match *interaction {
            Interaction::Clicked => {
                clipboard.set_contents(&text.sections[0].value);
            }

            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn generate_fen(
    mut text_query: Query<&mut Text, With<FenText>>,
    active_pieces_query: Query<(
        &Piece,
        &Transform,
        With<ActivePiece>,
        Without<SourcePiece>,
        Without<SelectedPiece>,
    )>,
) {
    // let mut fen = "8/8/8/8/8/8/8/8".to_string();

    let mut occupied_positions: Vec<(Piece, Square)> = vec![];
    for (piece, transform, _, _, _) in active_pieces_query.iter() {
        if let Some(square) = get_square(transform.translation.x, transform.translation.y) {
            occupied_positions.push((piece.clone(), square));
        }
    }

    // let pieces_on_file: Vec<(Piece, Square)> = occupied_positions
    // .into_iter()
    // .filter(|(_, square)| square.file == *file)
    // .collect();

    let fen = &format!(
        "{}/{}/{}/{}/{}/{}/{}/{}",
        get_fen_for_file(&File::EIGHT, occupied_positions.clone()),
        get_fen_for_file(&File::SEVEN, occupied_positions.clone()),
        get_fen_for_file(&File::SIX, occupied_positions.clone()),
        get_fen_for_file(&File::FIVE, occupied_positions.clone()),
        get_fen_for_file(&File::FOUR, occupied_positions.clone()),
        get_fen_for_file(&File::THREE, occupied_positions.clone()),
        get_fen_for_file(&File::TWO, occupied_positions.clone()),
        get_fen_for_file(&File::ONE, occupied_positions.clone()),
    );
    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[0].value = fen.into();
    }
}

fn fen_str(p: Piece, curr_empty_count: usize) -> String {
    let mut res = String::new();
    if curr_empty_count != 0 {
        res += &format!("{}", curr_empty_count);
    }
    res += p.def.fen_str();
    res
}

fn piece_on_given_square(
    pieces_on_file: &Vec<(Piece, Square)>,
    given_square: Square,
) -> Option<Piece> {
    if let Some((piece, _)) = pieces_on_file.into_iter().find(|e| e.1 == given_square) {
        Some(piece.clone())
    } else {
        None
    }
}

fn get_fen_for_file(file: &File, occupied_positions: Vec<(Piece, Square)>) -> String {
    let mut fen_for_file = String::new();

    let pieces_on_file = occupied_positions
        .into_iter()
        .filter(|(_, square)| square.file == *file)
        .collect::<Vec<(Piece, Square)>>();

    let mut curr_empty_count: usize = 0;

    if let Some(p) = piece_on_given_square(
        &pieces_on_file,
        Square {
            rank: Rank::A,
            file: file.clone(),
        },
    ) {
        fen_for_file += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if let Some(p) = piece_on_given_square(
        &pieces_on_file,
        Square {
            rank: Rank::B,
            file: file.clone(),
        },
    ) {
        fen_for_file += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if let Some(p) = piece_on_given_square(
        &pieces_on_file,
        Square {
            rank: Rank::C,
            file: file.clone(),
        },
    ) {
        fen_for_file += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if let Some(p) = piece_on_given_square(
        &pieces_on_file,
        Square {
            rank: Rank::D,
            file: file.clone(),
        },
    ) {
        fen_for_file += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if let Some(p) = piece_on_given_square(
        &pieces_on_file,
        Square {
            rank: Rank::E,
            file: file.clone(),
        },
    ) {
        fen_for_file += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if let Some(p) = piece_on_given_square(
        &pieces_on_file,
        Square {
            rank: Rank::F,
            file: file.clone(),
        },
    ) {
        fen_for_file += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if let Some(p) = piece_on_given_square(
        &pieces_on_file,
        Square {
            rank: Rank::G,
            file: file.clone(),
        },
    ) {
        fen_for_file += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if let Some(p) = piece_on_given_square(
        &pieces_on_file,
        Square {
            rank: Rank::H,
            file: file.clone(),
        },
    ) {
        fen_for_file += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if curr_empty_count != 0 {
        fen_for_file += &format!("{}", curr_empty_count);
    }

    fen_for_file
}
