use crate::assets::TextAssets;
use bevy::prelude::*;
use indoc::indoc;

#[derive(Component)]
pub struct ControlsText;

#[derive(Component)]
pub struct TipsElement;

pub fn spawn(mut commands: Commands, text_assets: Res<TextAssets>) {
    let clear_color_hex_string = "69696b";
    let tips_text = indoc! {"
        controls
        --------------------
        left mouse click: pickup / place a piece
        x: remove a selected piece | shift + x: clear the board
        s: save position (overwrites) | shift + s: clear saved position
        r: restore saved position on the board
        cmd + c: copy current FEN to clipboard
        i: all pieces in 'initial' / starting positions
    "};
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(10.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                position: UiRect {
                    left: Val::Px(30.0),
                    bottom: Val::Px(570.0),
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
        .insert(TipsElement)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: tips_text.to_string(),
                            style: TextStyle {
                                font: text_assets.regular_font_handle.clone(),
                                font_size: 14.0,
                                color: Color::rgb(0.15, 0.15, 0.15),
                            },
                        }],
                        alignment: TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Left,
                        },
                    },
                    ..Default::default()
                })
                .insert(ControlsText);
        });
}
