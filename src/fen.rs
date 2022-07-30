use crate::{
    assets::TextAssets,
    board::{get_square, File, Rank, Square},
    pieces::{place_piece, ActivePiece, Kind, Piece, PieceMaterialHandles, Side},
    tips::TipsText,
    types::{Board, ButtonInteraction, WithActivePiece},
};
use bevy::prelude::*;
use bevy_mod_picking::PickableMesh;
use std::fmt::Write as _;

#[derive(Component)]
pub struct FenElement;

#[derive(Component)]
pub struct FenText;

#[derive(Component)]
pub struct CopyElement;

#[derive(Component)]
pub struct CopyText;

#[derive(Component)]
pub struct SavedFenState {
    pub text_entity: Option<Entity>,
    pub curr: String,
    pub saved: String,
}

impl Default for SavedFenState {
    fn default() -> Self {
        SavedFenState {
            text_entity: None,
            curr: "".into(),
            saved: "".into(),
        }
    }
}

pub fn toggle_save_position(
    mut commands: Commands,
    mut saved_fen: ResMut<SavedFenState>,
    fen_assets: Res<TextAssets>,
    keys: Res<Input<KeyCode>>,
) {
    let clear_color_hex_string = "69696b";
    let text_color_hex_string = "a1a1a1";
    if keys.just_pressed(KeyCode::S) {
        if let Some(entity) = saved_fen.text_entity {
            // clear saved
            commands.entity(entity).despawn_recursive();
            saved_fen.text_entity = None;
            saved_fen.saved = "".into();
        } else {
            // save
            let display_text = format!("saved position\n{}", saved_fen.curr.clone());
            let entity: Entity = commands
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(400.0), Val::Px(20.0)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position: Rect {
                            left: Val::Px(150.0),
                            bottom: Val::Px(80.0),
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
                .insert(FenElement)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: display_text.clone(),
                                style: TextStyle {
                                    font: fen_assets.italic_font_handle.clone(),
                                    font_size: 12.0,
                                    color: Color::hex(text_color_hex_string).unwrap_or_else(|_| {
                                        panic!(
                                            "couldn't make hex color from {}",
                                            text_color_hex_string
                                        )
                                    }),
                                },
                            }],
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        },
                        ..Default::default()
                    });
                })
                .id();
            saved_fen.text_entity = Some(entity);
            saved_fen.saved = saved_fen.curr.clone();
        }
    }
}

pub fn spawn(mut commands: Commands, fen_assets: Res<TextAssets>) {
    let txt_val = "FEN NOTATION";
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(400.0), Val::Px(20.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: Rect {
                    left: Val::Px(150.0),
                    bottom: Val::Px(120.0),
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
                            value: txt_val.to_string(),
                            style: TextStyle {
                                font: fen_assets.regular_font_handle.clone(),
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
                    left: Val::Px(150.0),
                    bottom: Val::Px(110.0),
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
        .insert(CopyElement)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "(click FEN to copy)".into(),
                            style: TextStyle {
                                font: fen_assets.italic_font_handle.clone(),
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
                })
                .insert(CopyText);
        });
}

type WithFenText = (With<FenText>, Without<TipsText>, Without<CopyText>);
type WithCopyText = (With<CopyText>, Without<TipsText>, Without<FenText>);

pub fn copy_to_clipboard(
    mut clipboard: ResMut<bevy_egui::EguiClipboard>,
    interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<FenElement>)>,
    mut fen_text_query: Query<&mut Text, WithFenText>,
    mut copy_text_query: Query<&mut Text, WithCopyText>,
) {
    let clicked_color_hex_string = "a1a1a1";
    let copied_color_hex_string = "f3f0f5";
    for (interaction, children) in interaction_query.iter() {
        if let (Ok(mut fen_text), Ok(mut copy_text)) = (
            fen_text_query.get_mut(children[0]),
            copy_text_query.get_single_mut(),
        ) {
            match *interaction {
                Interaction::Clicked => {
                    clipboard.set_contents(&fen_text.sections[0].value);
                    fen_text.sections[0].style.color = Color::hex(clicked_color_hex_string)
                        .unwrap_or_else(|_| {
                            panic!("couldn't make hex color from {}", clicked_color_hex_string)
                        });
                    copy_text.sections[0].value = "copied!".into();
                    copy_text.sections[0].style.color = Color::hex(copied_color_hex_string)
                        .unwrap_or_else(|_| {
                            panic!("couldn't make hex color from {}", copied_color_hex_string)
                        });
                }

                Interaction::None | Interaction::Hovered => {
                    fen_text.sections[0].style.color = Color::rgb(0.9, 0.9, 0.9);
                    copy_text.sections[0].value = "(click FEN to copy)".into();
                    copy_text.sections[0].style.color = Color::rgb(0.15, 0.15, 0.15);
                }
            }
        }
    }
}

pub fn populate_board_from_fen(
    saved_fen: Res<SavedFenState>,
    board: Res<Board>,
    piece_material_handles: Res<PieceMaterialHandles>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &Piece, With<PickableMesh>, WithActivePiece)>,
    keys: Res<Input<KeyCode>>,
) {
    if !keys.pressed(KeyCode::R) {
        return;
    }

    if saved_fen.text_entity.is_none() {
        // there's no saved position to restore
        return;
    }

    // clear board
    for (entity, _piece, _, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for (rank, fen) in saved_fen.saved.split('/').enumerate() {
        populate_for_fen_rank(
            8 - rank,
            fen,
            &board,
            &piece_material_handles,
            &mut commands,
            &mut meshes,
        );
    }
}

pub fn populate_for_fen_rank(
    rank: usize,
    fen: &str,
    board: &Res<Board>,
    piece_material_handles: &Res<PieceMaterialHandles>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let num_to_file = |i| match i {
        1 => 'a',
        2 => 'b',
        3 => 'c',
        4 => 'd',
        5 => 'e',
        6 => 'f',
        7 => 'g',
        _ => 'h',
    };

    let mut file_num = 1;

    for c in fen.chars() {
        if let Some(p) = piece_for_fen_char(c) {
            let pos = format!("{}{}", num_to_file(file_num), rank);
            let material_handle = p.material_handle((**piece_material_handles).clone());
            place_piece(
                &pos,
                p,
                ActivePiece,
                &material_handle,
                board,
                commands,
                meshes,
            );
            file_num += 1;
        } else if let Some(d) = c.to_digit(10) {
            if d < 9 && d > 0 {
                file_num += d;
            }
        }
    }
}

pub fn piece_for_fen_char(piece: char) -> Option<Side> {
    match piece {
        'q' => Some(Side::Black(Kind::Queen)),
        'k' => Some(Side::Black(Kind::King)),
        'b' => Some(Side::Black(Kind::Bishop)),
        'n' => Some(Side::Black(Kind::Knight)),
        'r' => Some(Side::Black(Kind::Rook)),
        'p' => Some(Side::Black(Kind::Pawn)),
        'Q' => Some(Side::White(Kind::Queen)),
        'K' => Some(Side::White(Kind::King)),
        'B' => Some(Side::White(Kind::Bishop)),
        'N' => Some(Side::White(Kind::Knight)),
        'R' => Some(Side::White(Kind::Rook)),
        'P' => Some(Side::White(Kind::Pawn)),
        _ => None,
    }
}
pub fn generate_fen(
    mut text_query: Query<&mut Text, WithFenText>,
    mut saved_fen_state: ResMut<SavedFenState>,
    active_pieces_query: Query<(&Piece, &Transform, WithActivePiece)>,
) {
    let mut occupied_positions: Vec<(Piece, Square)> = vec![];
    for (piece, transform, _) in active_pieces_query.iter() {
        if let Some(square) = get_square(transform.translation.x, transform.translation.y) {
            occupied_positions.push((piece.clone(), square));
        }
    }

    let fen = &format!(
        "{}/{}/{}/{}/{}/{}/{}/{}",
        get_fen_for_rank(&Rank::Eight, occupied_positions.clone()),
        get_fen_for_rank(&Rank::Seven, occupied_positions.clone()),
        get_fen_for_rank(&Rank::Six, occupied_positions.clone()),
        get_fen_for_rank(&Rank::Five, occupied_positions.clone()),
        get_fen_for_rank(&Rank::Four, occupied_positions.clone()),
        get_fen_for_rank(&Rank::Three, occupied_positions.clone()),
        get_fen_for_rank(&Rank::Two, occupied_positions.clone()),
        get_fen_for_rank(&Rank::One, occupied_positions.clone()),
    );

    saved_fen_state.curr = fen.clone();

    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[0].value = fen.into();
    }
}

fn fen_str(p: Piece, curr_empty_count: usize) -> String {
    let mut res = String::new();
    if curr_empty_count != 0 {
        let _ = write!(res, "{}", curr_empty_count);
    }
    res += p.def.fen_str();
    res
}

fn piece_on_given_square(
    pieces_on_file: &[(Piece, Square)],
    given_square: Square,
) -> Option<Piece> {
    if let Some((piece, _)) = pieces_on_file.iter().find(|e| e.1 == given_square) {
        Some(piece.clone())
    } else {
        None
    }
}

fn get_fen_for_rank(rank: &Rank, occupied_positions: Vec<(Piece, Square)>) -> String {
    let mut fen_for_rank = String::new();

    let pieces_on_rank = occupied_positions
        .into_iter()
        .filter(|(_, square)| square.rank == *rank)
        .collect::<Vec<(Piece, Square)>>();

    let mut curr_empty_count: usize = 0;

    if let Some(p) = piece_on_given_square(
        &pieces_on_rank,
        Square {
            file: File::A,
            rank: rank.clone(),
        },
    ) {
        fen_for_rank += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if let Some(p) = piece_on_given_square(
        &pieces_on_rank,
        Square {
            file: File::B,
            rank: rank.clone(),
        },
    ) {
        fen_for_rank += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if let Some(p) = piece_on_given_square(
        &pieces_on_rank,
        Square {
            file: File::C,
            rank: rank.clone(),
        },
    ) {
        fen_for_rank += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if let Some(p) = piece_on_given_square(
        &pieces_on_rank,
        Square {
            file: File::D,
            rank: rank.clone(),
        },
    ) {
        fen_for_rank += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if let Some(p) = piece_on_given_square(
        &pieces_on_rank,
        Square {
            file: File::E,
            rank: rank.clone(),
        },
    ) {
        fen_for_rank += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if let Some(p) = piece_on_given_square(
        &pieces_on_rank,
        Square {
            file: File::F,
            rank: rank.clone(),
        },
    ) {
        fen_for_rank += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if let Some(p) = piece_on_given_square(
        &pieces_on_rank,
        Square {
            file: File::G,
            rank: rank.clone(),
        },
    ) {
        fen_for_rank += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if let Some(p) = piece_on_given_square(
        &pieces_on_rank,
        Square {
            file: File::H,
            rank: rank.clone(),
        },
    ) {
        fen_for_rank += &fen_str(p, curr_empty_count);
        curr_empty_count = 0;
    } else {
        curr_empty_count += 1;
    }

    if curr_empty_count != 0 {
        let _ = &write!(fen_for_rank, "{}", curr_empty_count);
    }

    fen_for_rank
}
