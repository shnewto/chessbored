use crate::board::{ActiveSquareHighlight, LegalMoveHighlight};
use crate::types::Moves;
use crate::{
    assets::BoardAssets,
    board,
    board::get_square,
    camera::ChessCamera,
    types::{Board, WithActivePiece, WithSelectedPiece, WithSourcePiece},
};
use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    ui::FocusPolicy,
};
use bevy_mod_picking::*;
use shakmaty::{Chess, Position};

#[derive(Debug, Clone)]
pub enum Side {
    White(Kind),
    Black(Kind),
}

#[derive(Debug, Clone)]
pub enum Kind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Kind {
    pub fn to_shakmaty(kind: Kind) -> shakmaty::Role {
        match kind {
            Kind::Pawn => shakmaty::Role::Pawn,
            Kind::Rook => shakmaty::Role::Rook,
            Kind::Knight => shakmaty::Role::Knight,
            Kind::Bishop => shakmaty::Role::Bishop,
            Kind::Queen => shakmaty::Role::Queen,
            Kind::King => shakmaty::Role::King,
        }
    }
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Pawn
    }
}

impl Default for Side {
    fn default() -> Self {
        Side::Black(Kind::default())
    }
}

impl Side {
    pub fn fen_str(&self) -> &str {
        match self {
            Self::White(p) => match p {
                Kind::Queen => "Q",
                Kind::King => "K",
                Kind::Bishop => "B",
                Kind::Knight => "N",
                Kind::Rook => "R",
                Kind::Pawn => "P",
            },
            Self::Black(p) => match p {
                Kind::Queen => "q",
                Kind::King => "k",
                Kind::Bishop => "b",
                Kind::Knight => "n",
                Kind::Rook => "r",
                Kind::Pawn => "p",
            },
        }
    }

    pub fn material_handle(&self, handles: PieceMaterialHandles) -> Handle<ColorMaterial> {
        match self {
            Self::White(p) => match p {
                Kind::Queen => handles.wq_material_handle,
                Kind::King => handles.wk_material_handle,
                Kind::Bishop => handles.wb_material_handle,
                Kind::Knight => handles.wn_material_handle,
                Kind::Rook => handles.wr_material_handle,
                Kind::Pawn => handles.wp_material_handle,
            },
            Self::Black(p) => match p {
                Kind::Queen => handles.bq_material_handle,
                Kind::King => handles.bk_material_handle,
                Kind::Bishop => handles.bb_material_handle,
                Kind::Knight => handles.bn_material_handle,
                Kind::Rook => handles.br_material_handle,
                Kind::Pawn => handles.bp_material_handle,
            },
        }
    }
}

#[derive(Component, Debug, Clone, Default)]
pub struct Piece {
    pub def: Side,
    pub selected_translation: Option<Vec3>,
    pub selected_from: Option<board::Square>,
    pub sprite_handle: Handle<ColorMaterial>,
    pub stale: bool,
}

impl Piece {
    pub fn to_shakmaty(piece: Piece) -> shakmaty::Role {
        match piece.def {
            Side::White(k) => Kind::to_shakmaty(k),
            Side::Black(k) => Kind::to_shakmaty(k),
        }
    }
}

#[derive(Component, Debug, Clone, Default)]
pub struct PieceMaterialHandles {
    pub bq_material_handle: Handle<ColorMaterial>,
    pub bk_material_handle: Handle<ColorMaterial>,
    pub bb_material_handle: Handle<ColorMaterial>,
    pub bn_material_handle: Handle<ColorMaterial>,
    pub br_material_handle: Handle<ColorMaterial>,
    pub bp_material_handle: Handle<ColorMaterial>,

    pub wq_material_handle: Handle<ColorMaterial>,
    pub wk_material_handle: Handle<ColorMaterial>,
    pub wb_material_handle: Handle<ColorMaterial>,
    pub wn_material_handle: Handle<ColorMaterial>,
    pub wr_material_handle: Handle<ColorMaterial>,
    pub wp_material_handle: Handle<ColorMaterial>,
}

#[derive(Component, Debug, Default)]
pub struct ActivePiece;

#[derive(Component, Debug, Default)]
pub struct SourcePiece;

#[derive(Component, Debug, Default)]
pub struct SelectedPiece;

pub fn side_piece_selection(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    query: Query<(
        &Piece,
        &Transform,
        &Mesh2dHandle,
        With<PickableMesh>,
        WithSourcePiece,
    )>,
    selected_query: Query<(&Piece, With<PickableMesh>, WithSelectedPiece)>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    for event in events.iter() {
        if let (PickingEvent::Clicked(e), true) =
            (event, mouse_button_input.pressed(MouseButton::Left))
        {
            if let Ok((piece_selection, transform, mesh_handle, _, _)) = query.get(*e) {
                if let Ok((_, _, _)) = selected_query.get_single() {
                    // disable picking a piece when one's already in hand
                    return;
                }

                commands
                    .spawn_bundle(MaterialMesh2dBundle {
                        mesh: mesh_handle.clone(),
                        material: piece_selection.sprite_handle.clone(),
                        transform: Transform::from_xyz(
                            transform.translation.x,
                            transform.translation.y,
                            10.0,
                        ),
                        ..default()
                    })
                    .insert_bundle(PickableBundle {
                        focus_policy: FocusPolicy::Pass,
                        ..default()
                    })
                    .insert(Piece {
                        def: piece_selection.def.clone(),
                        selected_translation: Some(transform.translation),
                        sprite_handle: piece_selection.sprite_handle.clone(),
                        stale: false,
                        ..default()
                    })
                    .insert(SelectedPiece);
            }
        }
    }
}

pub fn selection(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    board: Res<Board>,
    board_assets: Res<BoardAssets>,
    moves: Res<Moves>,
    mut active_query: Query<(
        &mut Piece,
        &Transform,
        &Mesh2dHandle,
        With<PickableMesh>,
        WithActivePiece,
    )>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    for event in events.iter() {
        // picking up
        let mut has_legal_move = false;
        if let (PickingEvent::Clicked(e), true) =
            (event, mouse_button_input.pressed(MouseButton::Left))
        {
            if let Ok((mut active_piece, active_transform, active_mesh, _, _)) =
                active_query.get_mut(*e)
            {
                if let Some(square) = get_square(
                    active_transform.translation.x,
                    active_transform.translation.y,
                ) {
                    if moves.in_use {
                        let available_moves: Vec<shakmaty::Move> = moves
                            .data
                            .legal_moves()
                            .into_iter()
                            .filter(|m| {
                                m.role() == Piece::to_shakmaty((*active_piece).clone())
                                    && m.from() == Some(board::Square::to_shakmaty(square.clone()))
                            })
                            .collect();
                        for m in available_moves {
                            if let Some(location) =
                                board::Square::location_from_shakmaty(m.to(), &board)
                            {
                                has_legal_move = true;
                                commands
                                    .spawn_bundle(SpriteBundle {
                                        texture: board_assets.legal_move_square_handle.clone(),
                                        transform: Transform::from_xyz(location.x, location.y, 1.0),
                                        ..default()
                                    })
                                    .insert(LegalMoveHighlight { stale: false });
                            }
                        }
                    }

                    if has_legal_move || !moves.in_use {
                        commands
                            .spawn_bundle(SpriteBundle {
                                texture: board_assets.square_selected_handle.clone(),
                                transform: Transform::from_xyz(
                                    active_transform.translation.x,
                                    active_transform.translation.y,
                                    1.0,
                                ),
                                ..default()
                            })
                            .insert(ActiveSquareHighlight {
                                stale: false,
                                square: Some(square.clone()),
                            });

                        commands
                            .spawn_bundle(MaterialMesh2dBundle {
                                mesh: active_mesh.clone(),
                                transform: Transform::from_xyz(
                                    active_transform.translation.x,
                                    active_transform.translation.y,
                                    10.0,
                                ),
                                material: active_piece.sprite_handle.clone(),
                                ..default()
                            })
                            .insert_bundle(PickableBundle {
                                focus_policy: FocusPolicy::Pass,
                                ..default()
                            })
                            .insert(Piece {
                                def: active_piece.def.clone(),
                                selected_translation: Some(active_transform.translation),
                                sprite_handle: active_piece.sprite_handle.clone(),
                                selected_from: Some(square),
                                stale: false,
                            })
                            .insert(SelectedPiece);
                        active_piece.stale = true;
                    }
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn drop_piece(
    board: Res<Board>,
    mut commands: Commands,
    mut moves: ResMut<Moves>,
    mut active_query: Query<(
        &mut Piece,
        &Transform,
        &Mesh2dHandle,
        With<PickableMesh>,
        WithActivePiece,
    )>,
    mut active_square_highlight_query: Query<&mut ActiveSquareHighlight>,
    mut selected_query: Query<(
        &mut Piece,
        &Transform,
        &Mesh2dHandle,
        With<PickableMesh>,
        WithSelectedPiece,
    )>,
    mut legal_move_square_hightlight_query: Query<(Entity, &mut LegalMoveHighlight)>,
    mut mouse_button_input: ResMut<Input<MouseButton>>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        if let Ok((mut selected_piece, selected_transform, selected_mesh, _, _)) =
            selected_query.get_single_mut()
        {
            if let Ok(mut square) = active_square_highlight_query.get_single_mut() {
                square.stale = true;
            }

            let (updated_x, updated_y) = if let Some(square) = get_square(
                selected_transform.translation.x,
                selected_transform.translation.y,
            ) {
                (
                    board.get(square.to_string().as_str()).unwrap().x,
                    board.get(square.to_string().as_str()).unwrap().y,
                )
            } else {
                selected_piece.stale = true;
                for (_entity, mut square) in legal_move_square_hightlight_query.iter_mut() {
                    square.stale = true;
                }
                return;
            };

            for (mut active_piece, active_transform, _, _, _) in active_query.iter_mut() {
                let selected_square = get_square(
                    selected_transform.translation.x,
                    selected_transform.translation.y,
                );

                let active_square = get_square(
                    active_transform.translation.x,
                    active_transform.translation.y,
                );

                if let (Some(from_square), Some(to_square)) = (
                    selected_piece.selected_from.clone(),
                    selected_square.clone(),
                ) {
                    let mut capture = None;
                    if Some(to_square.clone()) == active_square {
                        capture = Some(Piece::to_shakmaty((*active_piece).clone()));
                        active_piece.stale = true;
                    }

                    if moves.in_use {
                        let role = Piece::to_shakmaty((*selected_piece).clone());
                        let from = board::Square::to_shakmaty(from_square.clone());
                        let to = board::Square::to_shakmaty(to_square.clone());

                        let promotion = None;
                        let mv = shakmaty::Move::Normal {
                            role,
                            from,
                            to,
                            capture,
                            promotion,
                        };

                        let res = moves.data.clone().play(&mv);
                        if let Ok(m) = res {
                            moves.data = m;
                        } else {
                            selected_piece.stale = true;
                            for (_entity, mut square) in
                                legal_move_square_hightlight_query.iter_mut()
                            {
                                square.stale = true;
                            }
                        }
                    }
                }
            }
            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: selected_mesh.clone(),
                    transform: Transform::from_xyz(updated_x, updated_y, 5.0),
                    material: selected_piece.sprite_handle.clone(),
                    ..default()
                })
                .insert_bundle(PickableBundle {
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                })
                .insert(Piece {
                    def: selected_piece.def.clone(),
                    selected_translation: Some(Vec3::new(updated_x, updated_y, 5.0)),
                    sprite_handle: selected_piece.sprite_handle.clone(),
                    selected_from: get_square(
                        selected_transform.translation.x,
                        selected_transform.translation.y,
                    ),
                    stale: false,
                })
                .insert(ActivePiece);
            selected_piece.stale = true;

            for (_entity, mut square) in legal_move_square_hightlight_query.iter_mut() {
                square.stale = true;
            }
        }
    }
    mouse_button_input.clear_just_released(MouseButton::Left);
}

pub fn piece_movement(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform, With<ChessCamera>)>,
    mut query: Query<&mut Transform, (With<PickableMesh>, WithSelectedPiece)>,
) {
    for mut transform in query.iter_mut() {
        let (camera, camera_transform, _) = q_camera.single();

        let wnd = if let RenderTarget::Window(id) = camera.target {
            wnds.get(id).unwrap()
        } else {
            wnds.get_primary().unwrap()
        };

        if let Some(screen_pos) = wnd.cursor_position() {
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
            // let world_pos = ndc_to_world.project_point3(ndc.extend(-10.0));

            transform.translation.x = world_pos.x;
            transform.translation.y = world_pos.y;
        }
    }
}

pub fn cancel_piece_movement(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Piece,
        &Mesh2dHandle,
        With<PickableMesh>,
        WithSelectedPiece,
    )>,
    mut active_square_highlight_query: Query<(Entity, &mut ActiveSquareHighlight)>,
    mut legal_move_square_highlight_query: Query<(Entity, &mut LegalMoveHighlight)>,
    keys: Res<Input<KeyCode>>,
) {
    for (entity, piece, mesh, _, _) in query.iter_mut() {
        if keys.pressed(KeyCode::Escape) {
            if let Some(selected_translation) = piece.selected_translation {
                commands
                    .spawn_bundle(MaterialMesh2dBundle {
                        mesh: mesh.clone(),
                        transform: Transform::from_translation(selected_translation),
                        material: piece.sprite_handle.clone(),
                        ..default()
                    })
                    .insert_bundle(PickableBundle {
                        focus_policy: FocusPolicy::Pass,
                        ..default()
                    })
                    .insert(Piece {
                        def: piece.def.clone(),
                        selected_translation: Some(selected_translation),
                        sprite_handle: piece.sprite_handle.clone(),
                        stale: false,
                        ..default()
                    })
                    .insert(ActivePiece);
                commands.entity(entity).despawn_recursive();
            } else {
                commands.entity(entity).despawn_recursive();
            }

            for (_entity, mut square) in legal_move_square_highlight_query.iter_mut() {
                square.stale = true;
            }

            for (_entity, mut square) in active_square_highlight_query.iter_mut() {
                square.stale = true;
            }
        } else if keys.pressed(KeyCode::X)
            && !(keys.pressed(KeyCode::LShift) || keys.pressed(KeyCode::RShift))
        {
            commands.entity(entity).despawn_recursive();

            for (_entity, mut square) in legal_move_square_highlight_query.iter_mut() {
                square.stale = true;
            }

            for (_entity, mut square) in active_square_highlight_query.iter_mut() {
                square.stale = true;
            }
        }
    }
}

pub fn clear_board(
    mut commands: Commands,
    mut active_query: Query<(Entity, &Piece, With<PickableMesh>, WithActivePiece)>,
    mut selected_query: Query<(Entity, &Piece, With<PickableMesh>, WithSelectedPiece)>,
    mut active_square_highlight_query: Query<(Entity, &ActiveSquareHighlight)>,
    mut legal_move_square_highlight_query: Query<(Entity, &LegalMoveHighlight)>,
    keys: Res<Input<KeyCode>>,
) {
    for (entity, piece, _, _) in active_query.iter_mut() {
        if (keys.pressed(KeyCode::X)
            && (keys.pressed(KeyCode::LShift) || keys.pressed(KeyCode::RShift)))
            || piece.stale
        {
            commands.entity(entity).despawn_recursive();
        }
    }

    for (entity, piece, _, _) in selected_query.iter_mut() {
        if piece.stale {
            commands.entity(entity).despawn_recursive();
        }
    }

    for (entity, square) in active_square_highlight_query.iter_mut() {
        if square.stale {
            commands.entity(entity).despawn_recursive();
        }
    }

    for (entity, square) in legal_move_square_highlight_query.iter_mut() {
        if square.stale {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn set_sprite_handles(
    assets: ResMut<BoardAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut piece_material_handles: ResMut<PieceMaterialHandles>,
) {
    // sprites
    piece_material_handles.bq_material_handle = materials.add(assets.bq.clone().into());
    piece_material_handles.bk_material_handle = materials.add(assets.bk.clone().into());
    piece_material_handles.bb_material_handle = materials.add(assets.bb.clone().into());
    piece_material_handles.bn_material_handle = materials.add(assets.bn.clone().into());
    piece_material_handles.br_material_handle = materials.add(assets.br.clone().into());
    piece_material_handles.bp_material_handle = materials.add(assets.bp.clone().into());

    piece_material_handles.wq_material_handle = materials.add(assets.wq.clone().into());
    piece_material_handles.wk_material_handle = materials.add(assets.wk.clone().into());
    piece_material_handles.wb_material_handle = materials.add(assets.wb.clone().into());
    piece_material_handles.wn_material_handle = materials.add(assets.wn.clone().into());
    piece_material_handles.wr_material_handle = materials.add(assets.wr.clone().into());
    piece_material_handles.wp_material_handle = materials.add(assets.wp.clone().into());
}

pub fn place_piece<C: Component>(
    pos: &str,
    side: Side,
    piece_kind: C,
    material_handle: &Handle<ColorMaterial>,
    board: &Res<Board>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let pz = 0.0;
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get(pos).unwrap().x,
                board.get(pos).unwrap().y,
                pz,
            ),
            material: material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: side,
            sprite_handle: material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get(pos).unwrap().x,
                board.get(pos).unwrap().y,
                pz,
            )),
            stale: false,
            ..default()
        })
        .insert(piece_kind);
}

pub fn setup_piece_selection(
    board: Res<Board>,
    piece_material_handles: Res<PieceMaterialHandles>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // row 1 black
    place_piece(
        "bq",
        Side::Black(Kind::Queen),
        SourcePiece,
        &piece_material_handles.bq_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "bk",
        Side::Black(Kind::King),
        SourcePiece,
        &piece_material_handles.bk_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    // row 2 black
    place_piece(
        "bp",
        Side::Black(Kind::Pawn),
        SourcePiece,
        &piece_material_handles.bp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "bb",
        Side::Black(Kind::Bishop),
        SourcePiece,
        &piece_material_handles.bb_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    // row 3 black
    place_piece(
        "bn",
        Side::Black(Kind::Knight),
        SourcePiece,
        &piece_material_handles.bn_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "br",
        Side::Black(Kind::Rook),
        SourcePiece,
        &piece_material_handles.br_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );

    // row 1 white
    place_piece(
        "wq",
        Side::White(Kind::Queen),
        SourcePiece,
        &piece_material_handles.wq_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "wk",
        Side::White(Kind::King),
        SourcePiece,
        &piece_material_handles.wk_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    // row 2 white
    place_piece(
        "wp",
        Side::White(Kind::Pawn),
        SourcePiece,
        &piece_material_handles.wp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "wb",
        Side::White(Kind::Bishop),
        SourcePiece,
        &piece_material_handles.wb_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    // row 3 white
    place_piece(
        "wn",
        Side::White(Kind::Knight),
        SourcePiece,
        &piece_material_handles.wn_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "wr",
        Side::White(Kind::Rook),
        SourcePiece,
        &piece_material_handles.wr_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
}

pub fn starting_positions(
    board: Res<Board>,
    mut moves: ResMut<Moves>,
    mut commands: Commands,
    piece_material_handles: Res<PieceMaterialHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &Piece, With<PickableMesh>, WithActivePiece)>,
    keys: Res<Input<KeyCode>>,
) {
    if !keys.pressed(KeyCode::I) {
        return;
    }

    moves.data = Chess::default();
    moves.in_use = true;

    for (entity, _piece, _, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // black pawns
    place_piece(
        "a7",
        Side::Black(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.bp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "b7",
        Side::Black(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.bp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "c7",
        Side::Black(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.bp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "d7",
        Side::Black(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.bp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "e7",
        Side::Black(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.bp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "f7",
        Side::Black(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.bp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "g7",
        Side::Black(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.bp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "h7",
        Side::Black(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.bp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );

    // black major/minor
    place_piece(
        "a8",
        Side::Black(Kind::Rook),
        ActivePiece,
        &piece_material_handles.br_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "b8",
        Side::Black(Kind::Knight),
        ActivePiece,
        &piece_material_handles.bn_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "c8",
        Side::Black(Kind::Bishop),
        ActivePiece,
        &piece_material_handles.bb_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );

    place_piece(
        "d8",
        Side::Black(Kind::Queen),
        ActivePiece,
        &piece_material_handles.bq_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );

    place_piece(
        "e8",
        Side::Black(Kind::King),
        ActivePiece,
        &piece_material_handles.bk_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );

    place_piece(
        "f8",
        Side::Black(Kind::Bishop),
        ActivePiece,
        &piece_material_handles.bb_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );

    place_piece(
        "g8",
        Side::Black(Kind::Knight),
        ActivePiece,
        &piece_material_handles.bn_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );

    place_piece(
        "h8",
        Side::Black(Kind::Rook),
        ActivePiece,
        &piece_material_handles.br_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );

    // white pawns
    place_piece(
        "a2",
        Side::White(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.wp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "b2",
        Side::White(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.wp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "c2",
        Side::White(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.wp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "d2",
        Side::White(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.wp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "e2",
        Side::White(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.wp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "f2",
        Side::White(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.wp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "g2",
        Side::White(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.wp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "h2",
        Side::White(Kind::Pawn),
        ActivePiece,
        &piece_material_handles.wp_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );

    // white major/minor
    place_piece(
        "a1",
        Side::White(Kind::Rook),
        ActivePiece,
        &piece_material_handles.wr_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "b1",
        Side::White(Kind::Knight),
        ActivePiece,
        &piece_material_handles.wn_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
    place_piece(
        "c1",
        Side::White(Kind::Bishop),
        ActivePiece,
        &piece_material_handles.wb_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );

    place_piece(
        "d1",
        Side::White(Kind::Queen),
        ActivePiece,
        &piece_material_handles.wq_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );

    place_piece(
        "e1",
        Side::White(Kind::King),
        ActivePiece,
        &piece_material_handles.wk_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );

    place_piece(
        "f1",
        Side::White(Kind::Bishop),
        ActivePiece,
        &piece_material_handles.wb_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );

    place_piece(
        "g1",
        Side::White(Kind::Knight),
        ActivePiece,
        &piece_material_handles.wn_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );

    place_piece(
        "h1",
        Side::White(Kind::Rook),
        ActivePiece,
        &piece_material_handles.wr_material_handle,
        &board,
        &mut commands,
        &mut meshes,
    );
}
