use bevy::prelude::*;

use crate::assets::FenAssets;

#[derive(Component)]
pub struct Fen;

pub fn spawn(mut commands: Commands, fen_assets: Res<FenAssets>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(400.0), Val::Px(20.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                position: Rect {
                    left: Val::Px(160.0),
                    bottom: Val::Px(70.0),
                    ..default()
                },
                ..default()
            },
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..Default::default()
        })
        .insert(Fen)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "FEN NOTATION".to_string(),
                        style: TextStyle {
                            font: fen_assets.fen_font_handle.clone(),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
        
        let clear_color_hex_string = "69696b";
        
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(400.0), Val::Px(10.0)),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    position: Rect {
                        left: Val::Px(160.0),
                        bottom: Val::Px(60.0),
                        ..default()
                    },
                    ..default()
                },
                color: Color::hex(clear_color_hex_string).unwrap_or_else(|_| {
                    panic!("couldn't make hex color from {}", clear_color_hex_string)
                }).into(),
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
                        alignment: Default::default(),
                    },
                    ..Default::default()
                });
            });
}

type ButtonInteraction<'a> = (&'a Interaction, &'a Children);

pub fn copy_to_clipboard(
    mut clipboard: ResMut<bevy_egui::EguiClipboard>,
    interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<Fen>)>,
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
