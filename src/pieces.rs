use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    ui::FocusPolicy,
};
use bevy_mod_picking::*;

use crate::{
    assets::BoardAssets,
    board::get_square,
    camera::ChessCamera,
    types::{Board, WithActivePiece, WithSelectedPiece, WithSourcePiece},
};

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
}
#[derive(Component, Debug, Clone, Default)]
pub struct Piece {
    pub def: Side,
    pub selected_translation: Option<Vec3>,
    pub sprite_handle: Handle<ColorMaterial>,
    pub stale: bool,
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
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
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
                            0.1,
                        ),
                        ..default()
                    })
                    .insert_bundle(PickableBundle {
                        focus_policy: FocusPolicy::Pass,
                        ..default()
                    })
                    .insert(Piece {
                        def: piece_selection.def.clone(),
                        selected_translation: None,
                        sprite_handle: piece_selection.sprite_handle.clone(),
                        ..default()
                    })
                    .insert(SelectedPiece);
            }
        }
    }
}

pub fn selection(
    board: Res<Board>,
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mut active_query: Query<(
        &mut Piece,
        &Transform,
        &Mesh2dHandle,
        With<PickableMesh>,
        WithActivePiece,
    )>,
    mut selected_query: Query<(
        &mut Piece,
        &Transform,
        &Mesh2dHandle,
        With<PickableMesh>,
        WithSelectedPiece,
    )>,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            // we're clicking on a placed piece
            if let Ok((mut active_piece, active_transform, active_mesh, _, _)) =
                active_query.get_mut(*e)
            {
                if let Ok((mut selected_piece, selected_transform, selected_mesh, _, _)) =
                    selected_query.get_single_mut()
                {
                    // there's a piece selected / in hand already

                    if selected_transform.translation.x > 360.0
                        || selected_transform.translation.y < -10.0
                    {
                        // don't allow grabbing more pieces from the when an piece is already in hand
                        return;
                    }

                    let (updated_x, updated_y) = if let Some(square) = get_square(
                        selected_transform.translation.x,
                        selected_transform.translation.y,
                    ) {
                        (
                            board.get(&*square.to_string()).unwrap().x,
                            board.get(&*square.to_string()).unwrap().y,
                        )
                    } else {
                        (
                            selected_transform.translation.x,
                            selected_transform.translation.y,
                        )
                    };

                    commands
                        .spawn_bundle(MaterialMesh2dBundle {
                            mesh: selected_mesh.clone(),
                            transform: Transform::from_xyz(updated_x, updated_y, 0.0),
                            material: selected_piece.sprite_handle.clone(),
                            ..default()
                        })
                        .insert_bundle(PickableBundle {
                            focus_policy: FocusPolicy::Pass,
                            ..default()
                        })
                        .insert(Piece {
                            def: selected_piece.def.clone(),
                            selected_translation: None,
                            sprite_handle: selected_piece.sprite_handle.clone(),
                            ..default()
                        })
                        .insert(ActivePiece);
                    active_piece.stale = true;
                    selected_piece.stale = true;
                } else {
                    // there's no piece in hand so put the current selection in hand
                    commands
                        .spawn_bundle(MaterialMesh2dBundle {
                            mesh: active_mesh.clone(),
                            transform: Transform::from_xyz(
                                active_transform.translation.x,
                                active_transform.translation.y,
                                0.1,
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
                            ..default()
                        })
                        .insert(SelectedPiece);
                    active_piece.stale = true;
                }
            }
            // there's no piece on the board, only one in hand
            else if let Ok((mut selected_piece, selected_transform, selected_mesh, _, _)) =
                selected_query.get_single_mut()
            {
                if selected_transform.translation.x > 360.0
                    || selected_transform.translation.y < -10.0
                {
                    // don't allow placing on the right side of the board where the piece selections are
                    return;
                }

                let (updated_x, updated_y) = if let Some(square) = get_square(
                    selected_transform.translation.x,
                    selected_transform.translation.y,
                ) {
                    (
                        board.get(&*square.to_string()).unwrap().x,
                        board.get(&*square.to_string()).unwrap().y,
                    )
                } else {
                    (
                        selected_transform.translation.x,
                        selected_transform.translation.y,
                    )
                };

                commands
                    .spawn_bundle(MaterialMesh2dBundle {
                        mesh: selected_mesh.clone(),
                        transform: Transform::from_xyz(updated_x, updated_y, 0.0),
                        material: selected_piece.sprite_handle.clone(),
                        ..default()
                    })
                    .insert_bundle(PickableBundle {
                        focus_policy: FocusPolicy::Pass,
                        ..default()
                    })
                    .insert(Piece {
                        def: selected_piece.def.clone(),
                        selected_translation: None,
                        sprite_handle: selected_piece.sprite_handle.clone(),
                        ..default()
                    })
                    .insert(ActivePiece);
                selected_piece.stale = true;
            }
        }
    }
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
                camera_transform.compute_matrix() * camera.projection_matrix.inverse();

            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

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
                        ..default()
                    })
                    .insert(ActivePiece);
                commands.entity(entity).despawn_recursive();
            } else {
                commands.entity(entity).despawn_recursive();
            }
        } else if keys.pressed(KeyCode::X) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn clear_board(
    mut commands: Commands,
    mut active_query: Query<(Entity, &Piece, With<PickableMesh>, WithActivePiece)>,
    mut selected_query: Query<(Entity, &Piece, With<PickableMesh>, WithSelectedPiece)>,
    keys: Res<Input<KeyCode>>,
) {
    for (entity, piece, _, _) in active_query.iter_mut() {
        if keys.pressed(KeyCode::C) || piece.stale {
            commands.entity(entity).despawn_recursive();
        }
    }

    for (entity, piece, _, _) in selected_query.iter_mut() {
        if keys.pressed(KeyCode::C) || piece.stale {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn setup_piece_selection(
    mut commands: Commands,
    assets: ResMut<BoardAssets>,
    board: Res<Board>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let pz = 0.0;

    // sprites
    let bq_material_handle = materials.add(assets.bq.clone().into());
    let bk_material_handle = materials.add(assets.bk.clone().into());
    let bb_material_handle = materials.add(assets.bb.clone().into());
    let bn_material_handle = materials.add(assets.bn.clone().into());
    let br_material_handle = materials.add(assets.br.clone().into());
    let bp_material_handle = materials.add(assets.bp.clone().into());

    let wq_material_handle = materials.add(assets.wq.clone().into());
    let wk_material_handle = materials.add(assets.wk.clone().into());
    let wb_material_handle = materials.add(assets.wb.clone().into());
    let wn_material_handle = materials.add(assets.wn.clone().into());
    let wr_material_handle = materials.add(assets.wr.clone().into());
    let wp_material_handle = materials.add(assets.wp.clone().into());

    // row 1 black
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("bq").unwrap().x,
                board.get("bq").unwrap().y,
                pz,
            ),
            material: bq_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Queen),
            sprite_handle: bq_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("bq").unwrap().x,
                board.get("bq").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(SourcePiece);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("bk").unwrap().x,
                board.get("bk").unwrap().y,
                pz,
            ),
            material: bk_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::King),
            sprite_handle: bk_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("bk").unwrap().x,
                board.get("bk").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(SourcePiece);
    // row 2 black
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("bp").unwrap().x,
                board.get("bp").unwrap().y,
                pz,
            ),
            material: bp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Pawn),
            sprite_handle: bp_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("bp").unwrap().x,
                board.get("bp").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(SourcePiece);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("bb").unwrap().x,
                board.get("bb").unwrap().y,
                pz,
            ),
            material: bb_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Bishop),
            sprite_handle: bb_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("bb").unwrap().x,
                board.get("bb").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(SourcePiece);

    // row3 black
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("bn").unwrap().x,
                board.get("bn").unwrap().y,
                pz,
            ),
            material: bn_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Knight),
            sprite_handle: bn_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("bn").unwrap().x,
                board.get("bn").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(SourcePiece);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("br").unwrap().x,
                board.get("br").unwrap().y,
                pz,
            ),
            material: br_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Rook),
            sprite_handle: br_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("br").unwrap().x,
                board.get("br").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(SourcePiece);

    // row 1 white
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("wq").unwrap().x,
                board.get("wq").unwrap().y,
                pz,
            ),
            material: wq_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Queen),
            sprite_handle: wq_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("wq").unwrap().x,
                board.get("wq").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(SourcePiece);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("wk").unwrap().x,
                board.get("wk").unwrap().y,
                pz,
            ),
            material: wk_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::King),
            sprite_handle: wk_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("wk").unwrap().x,
                board.get("wk").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(SourcePiece);

    // row 2 white
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("wp").unwrap().x,
                board.get("wp").unwrap().y,
                pz,
            ),
            material: wp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Pawn),
            sprite_handle: wp_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("wp").unwrap().x,
                board.get("wp").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(SourcePiece);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("wb").unwrap().x,
                board.get("wb").unwrap().y,
                pz,
            ),
            material: wb_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Bishop),
            sprite_handle: wb_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("wb").unwrap().x,
                board.get("wb").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(SourcePiece);
    // row 3 white
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("wn").unwrap().x,
                board.get("wn").unwrap().y,
                pz,
            ),
            material: wn_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Knight),
            sprite_handle: wn_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("wn").unwrap().x,
                board.get("wn").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(SourcePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("wr").unwrap().x,
                board.get("wr").unwrap().y,
                pz,
            ),
            material: wr_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Rook),
            sprite_handle: wr_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("wr").unwrap().x,
                board.get("wr").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(SourcePiece);
}

pub fn starting_positions(
    mut commands: Commands,
    assets: ResMut<BoardAssets>,
    board: Res<Board>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &Piece, With<PickableMesh>, WithActivePiece)>,
    keys: Res<Input<KeyCode>>,
) {
    if !keys.pressed(KeyCode::I) {
        return;
    }

    for (entity, _piece, _, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let pz = 0.0;

    // sprites
    let bq_material_handle = materials.add(assets.bq.clone().into());
    let bk_material_handle = materials.add(assets.bk.clone().into());
    let bb_material_handle = materials.add(assets.bb.clone().into());
    let bn_material_handle = materials.add(assets.bn.clone().into());
    let br_material_handle = materials.add(assets.br.clone().into());
    let bp_material_handle = materials.add(assets.bp.clone().into());

    let wq_material_handle = materials.add(assets.wq.clone().into());
    let wk_material_handle = materials.add(assets.wk.clone().into());
    let wb_material_handle = materials.add(assets.wb.clone().into());
    let wn_material_handle = materials.add(assets.wn.clone().into());
    let wr_material_handle = materials.add(assets.wr.clone().into());
    let wp_material_handle = materials.add(assets.wp.clone().into());

    // black pawns
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("a7").unwrap().x,
                board.get("a7").unwrap().y,
                pz,
            ),
            material: bp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Pawn),
            sprite_handle: bp_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("a7").unwrap().x,
                board.get("a7").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("b7").unwrap().x,
                board.get("b7").unwrap().y,
                pz,
            ),
            material: bp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Pawn),
            sprite_handle: bp_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("b7").unwrap().x,
                board.get("b7").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("c7").unwrap().x,
                board.get("c7").unwrap().y,
                pz,
            ),
            material: bp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Pawn),
            sprite_handle: bp_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("c7").unwrap().x,
                board.get("c7").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("d7").unwrap().x,
                board.get("d7").unwrap().y,
                pz,
            ),
            material: bp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Pawn),
            sprite_handle: bp_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("d7").unwrap().x,
                board.get("d7").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("e7").unwrap().x,
                board.get("e7").unwrap().y,
                pz,
            ),
            material: bp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Pawn),
            sprite_handle: bp_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("e7").unwrap().x,
                board.get("e7").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("f7").unwrap().x,
                board.get("f7").unwrap().y,
                pz,
            ),
            material: bp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Pawn),
            sprite_handle: bp_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("f7").unwrap().x,
                board.get("f7").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("g7").unwrap().x,
                board.get("g7").unwrap().y,
                pz,
            ),
            material: bp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Pawn),
            sprite_handle: bp_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("g7").unwrap().x,
                board.get("g7").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("h7").unwrap().x,
                board.get("h7").unwrap().y,
                pz,
            ),
            material: bp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Pawn),
            sprite_handle: bp_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("h7").unwrap().x,
                board.get("h7").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    // black major/minor
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("a8").unwrap().x,
                board.get("a8").unwrap().y,
                pz,
            ),
            material: br_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Rook),
            sprite_handle: br_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("a8").unwrap().x,
                board.get("a8").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("b8").unwrap().x,
                board.get("b8").unwrap().y,
                pz,
            ),
            material: bn_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Knight),
            sprite_handle: bn_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("b8").unwrap().x,
                board.get("b8").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("c8").unwrap().x,
                board.get("c8").unwrap().y,
                pz,
            ),
            material: bb_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Bishop),
            sprite_handle: bb_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("c8").unwrap().x,
                board.get("c8").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("d8").unwrap().x,
                board.get("d8").unwrap().y,
                pz,
            ),
            material: bq_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Queen),
            sprite_handle: bq_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("d8").unwrap().x,
                board.get("d8").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("e8").unwrap().x,
                board.get("e8").unwrap().y,
                pz,
            ),
            material: bk_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::King),
            sprite_handle: bk_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("e8").unwrap().x,
                board.get("e8").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("f8").unwrap().x,
                board.get("f8").unwrap().y,
                pz,
            ),
            material: bb_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Bishop),
            sprite_handle: bb_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("f8").unwrap().x,
                board.get("f8").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("g8").unwrap().x,
                board.get("g8").unwrap().y,
                pz,
            ),
            material: bn_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Knight),
            sprite_handle: bn_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("g8").unwrap().x,
                board.get("g8").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("h8").unwrap().x,
                board.get("h8").unwrap().y,
                pz,
            ),
            material: br_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::Black(Kind::Rook),
            sprite_handle: br_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("h8").unwrap().x,
                board.get("h8").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    // white pawns
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("a2").unwrap().x,
                board.get("a2").unwrap().y,
                pz,
            ),
            material: wp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Pawn),
            sprite_handle: wp_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("a2").unwrap().x,
                board.get("a2").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("b2").unwrap().x,
                board.get("b2").unwrap().y,
                pz,
            ),
            material: wp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Pawn),
            sprite_handle: wp_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("b2").unwrap().x,
                board.get("b2").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("c2").unwrap().x,
                board.get("c2").unwrap().y,
                pz,
            ),
            material: wp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Pawn),
            sprite_handle: wp_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("c2").unwrap().x,
                board.get("c2").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("d2").unwrap().x,
                board.get("d2").unwrap().y,
                pz,
            ),
            material: wp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Pawn),
            sprite_handle: wp_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("d2").unwrap().x,
                board.get("d2").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("e2").unwrap().x,
                board.get("e2").unwrap().y,
                pz,
            ),
            material: wp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Pawn),
            sprite_handle: wp_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("e2").unwrap().x,
                board.get("e2").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("f2").unwrap().x,
                board.get("f2").unwrap().y,
                pz,
            ),
            material: wp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Pawn),
            sprite_handle: wp_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("f2").unwrap().x,
                board.get("f2").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("g2").unwrap().x,
                board.get("g2").unwrap().y,
                pz,
            ),
            material: wp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Pawn),
            sprite_handle: wp_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("g2").unwrap().x,
                board.get("g2").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("h2").unwrap().x,
                board.get("h2").unwrap().y,
                pz,
            ),
            material: wp_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Pawn),
            sprite_handle: wp_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("h2").unwrap().x,
                board.get("h2").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    // white major/minor
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("a1").unwrap().x,
                board.get("a1").unwrap().y,
                pz,
            ),
            material: wr_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Rook),
            sprite_handle: wr_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("a1").unwrap().x,
                board.get("a1").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("b1").unwrap().x,
                board.get("b1").unwrap().y,
                pz,
            ),
            material: wn_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Knight),
            sprite_handle: wn_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("b1").unwrap().x,
                board.get("b1").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("c1").unwrap().x,
                board.get("c1").unwrap().y,
                pz,
            ),
            material: wb_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Bishop),
            sprite_handle: wb_material_handle.clone(),
            selected_translation: Some(Vec3::new(
                board.get("c1").unwrap().x,
                board.get("c1").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("d1").unwrap().x,
                board.get("d1").unwrap().y,
                pz,
            ),
            material: wq_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Queen),
            sprite_handle: wq_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("d1").unwrap().x,
                board.get("d1").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("e1").unwrap().x,
                board.get("e1").unwrap().y,
                pz,
            ),
            material: wk_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::King),
            sprite_handle: wk_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("e1").unwrap().x,
                board.get("e1").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("f1").unwrap().x,
                board.get("f1").unwrap().y,
                pz,
            ),
            material: wb_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Bishop),
            sprite_handle: wb_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("f1").unwrap().x,
                board.get("f1").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("g1").unwrap().x,
                board.get("g1").unwrap().y,
                pz,
            ),
            material: wn_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Knight),
            sprite_handle: wn_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("g1").unwrap().x,
                board.get("g1").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(50.0, 50.0),
                    ..default()
                }))
                .into(),
            transform: Transform::from_xyz(
                board.get("h1").unwrap().x,
                board.get("h1").unwrap().y,
                pz,
            ),
            material: wr_material_handle.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle {
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(Piece {
            def: Side::White(Kind::Rook),
            sprite_handle: wr_material_handle,
            selected_translation: Some(Vec3::new(
                board.get("h1").unwrap().x,
                board.get("h1").unwrap().y,
                pz,
            )),
            ..default()
        })
        .insert(ActivePiece);
}
