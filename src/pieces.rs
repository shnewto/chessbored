use bevy::{prelude::*, render::camera::RenderTarget, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::*;

use crate::{assets::BoardAssets, board::Board, camera::ChessCamera};

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

#[derive(Component, Debug, Default)]
pub struct Piece {
    pub selected: bool,
    pub def: Side,
    pub selected_translation: Option<Vec3>,
}

#[derive(Component, Debug, Default)]
pub struct PieceSelection {
    pub def: Side,
    pub selected_translation: Option<Vec3>,
    pub sprite_handle: Handle<ColorMaterial>,
}

pub fn side_piece_selection(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut events: EventReader<PickingEvent>,
    query: Query<(&PieceSelection, &Transform), With<PickableMesh>>,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            if let Ok((piece_selection, transform)) = query.get(*e) {
                commands
                    .spawn_bundle(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Quad {
                                size: Vec2::new(50.0, 50.0),
                                ..default()
                            }))
                            .into(),
                        transform: Transform::from_xyz(
                            transform.translation.x,
                            transform.translation.y,
                            1.0,
                        ),
                        material: piece_selection.sprite_handle.clone(),
                        ..default()
                    })
                    .insert_bundle(PickableBundle::default())
                    .insert(Piece {
                        selected: true,
                        def: piece_selection.def.clone(),
                        selected_translation: None,
                    });
            }
        }
    }
}

pub fn selection(
    mut events: EventReader<PickingEvent>,
    mut query: Query<(&mut Piece, &mut Transform), With<PickableMesh>>,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            if let Ok((mut piece, mut transform)) = query.get_mut(*e) {
                piece.selected = !piece.selected;
                if piece.selected {
                    piece.selected_translation = Some(transform.translation);
                    transform.translation.z = 1.0;
                } else {
                    transform.translation.z = 0.0;
                }
            }
        }
    }
}

pub fn piece_movement(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<ChessCamera>>,
    mut query: Query<(&mut Transform, &Piece, With<PickableMesh>)>,
) {
    for (mut transform, piece, _) in query.iter_mut() {
        if piece.selected {
            let (camera, camera_transform) = q_camera.single();

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

                transform.translation.x = world_pos.x;
                transform.translation.y = world_pos.y;
            }
        }
    }
}

pub fn cancel_piece_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Piece, With<PickableMesh>)>,
    keys: Res<Input<KeyCode>>,
) {
    for (entity, mut transform, mut piece, _) in query.iter_mut() {
        if piece.selected {
            if keys.pressed(KeyCode::Escape) {
                piece.selected = false;
                if let Some(selected_translation) = piece.selected_translation {
                    transform.translation = selected_translation;
                } else {
                    commands.entity(entity).despawn();
                }
            } else if keys.pressed(KeyCode::X) {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn clear_board(
    mut commands: Commands,
    mut query: Query<(Entity, &Piece), With<PickableMesh>>,
    keys: Res<Input<KeyCode>>,
) {
    for (entity, _piece) in query.iter_mut() {
        if keys.pressed(KeyCode::C) {
            commands.entity(entity).despawn();
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

    // row 1 black
    let bq_material_handle = materials.add(assets.bq.clone().into());
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
        .insert_bundle(PickableBundle::default())
        .insert(PieceSelection {
            def: Side::Black(Kind::Queen),
            sprite_handle: bq_material_handle,
            ..default()
        });

    let bk_material_handle = materials.add(assets.bk.clone().into());
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
        .insert_bundle(PickableBundle::default())
        .insert(PieceSelection {
            def: Side::Black(Kind::King),
            sprite_handle: bk_material_handle,
            ..default()
        });
    // row 2 black
    let bp_material_handle = materials.add(assets.bp.clone().into());
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
        .insert_bundle(PickableBundle::default())
        .insert(PieceSelection {
            def: Side::Black(Kind::Pawn),
            sprite_handle: bp_material_handle,
            ..default()
        });
    let bb_material_handle = materials.add(assets.bb.clone().into());
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
        .insert_bundle(PickableBundle::default())
        .insert(PieceSelection {
            def: Side::Black(Kind::Bishop),
            sprite_handle: bb_material_handle,
            ..default()
        });
    // row3 black
    let bn_material_handle = materials.add(assets.bn.clone().into());
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
        .insert_bundle(PickableBundle::default())
        .insert(PieceSelection {
            def: Side::Black(Kind::Knight),
            sprite_handle: bn_material_handle,
            ..default()
        });
    let br_material_handle = materials.add(assets.br.clone().into());
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
        .insert_bundle(PickableBundle::default())
        .insert(PieceSelection {
            def: Side::Black(Kind::Rook),
            sprite_handle: br_material_handle,
            ..default()
        });

    // row 1 white
    let wq_material_handle = materials.add(assets.wq.clone().into());
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
        .insert_bundle(PickableBundle::default())
        .insert(PieceSelection {
            def: Side::White(Kind::Queen),
            sprite_handle: wq_material_handle,
            ..default()
        });

    let wk_material_handle = materials.add(assets.wk.clone().into());
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
        .insert_bundle(PickableBundle::default())
        .insert(PieceSelection {
            def: Side::White(Kind::King),
            sprite_handle: wk_material_handle,
            ..default()
        });
    // row 2 white
    let wp_material_handle = materials.add(assets.wp.clone().into());
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
        .insert_bundle(PickableBundle::default())
        .insert(PieceSelection {
            def: Side::White(Kind::Pawn),
            sprite_handle: wp_material_handle,
            ..default()
        });

    let wb_material_handle = materials.add(assets.wb.clone().into());
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
        .insert_bundle(PickableBundle::default())
        .insert(PieceSelection {
            def: Side::White(Kind::Bishop),
            sprite_handle: wb_material_handle,
            ..default()
        });
    // row 3 white
    let wn_material_handle = materials.add(assets.wn.clone().into());
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
        .insert_bundle(PickableBundle::default())
        .insert(PieceSelection {
            def: Side::White(Kind::Knight),
            sprite_handle: wn_material_handle,
            ..default()
        });
    let wr_material_handle = materials.add(assets.wr.clone().into());
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
        .insert_bundle(PickableBundle::default())
        .insert(PieceSelection {
            def: Side::White(Kind::Rook),
            sprite_handle: wr_material_handle,
            ..default()
        });
}

pub fn starting_positions(
    mut commands: Commands,
    assets: ResMut<BoardAssets>,
    board: Res<Board>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &Piece), With<PickableMesh>>,
    keys: Res<Input<KeyCode>>,
) {
    if !keys.pressed(KeyCode::I) {
        return;
    }

    for (entity, _piece) in query.iter() {
        commands.entity(entity).despawn();
    }

    let pz = 0.0;

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
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.bp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.br.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Rook),
            ..default()
        });
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
            material: materials.add(assets.bn.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Knight),
            ..default()
        });
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
            material: materials.add(assets.bb.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Bishop),
            ..default()
        });
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
            material: materials.add(assets.bq.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Queen),
            ..default()
        });
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
            material: materials.add(assets.bk.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::King),
            ..default()
        });
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
            material: materials.add(assets.bb.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Bishop),
            ..default()
        });
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
            material: materials.add(assets.bn.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Knight),
            ..default()
        });
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
            material: materials.add(assets.br.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::Black(Kind::Rook),
            ..default()
        });
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
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.wp.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Pawn),
            ..default()
        });
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
            material: materials.add(assets.wr.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Rook),
            ..default()
        });
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
            material: materials.add(assets.wn.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Knight),
            ..default()
        });
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
            material: materials.add(assets.wb.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Bishop),
            ..default()
        });
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
            material: materials.add(assets.wq.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Queen),
            ..default()
        });
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
            material: materials.add(assets.wk.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::King),
            ..default()
        });
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
            material: materials.add(assets.wb.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Bishop),
            ..default()
        });
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
            material: materials.add(assets.wn.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Knight),
            ..default()
        });
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
            material: materials.add(assets.wr.clone().into()),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Piece {
            selected: false,
            def: Side::White(Kind::Rook),
            ..default()
        });
}
