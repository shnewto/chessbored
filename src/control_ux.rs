use crate::assets::TextAssets;
use bevy::prelude::*;
use indoc::indoc;

#[derive(Component)]
pub struct ControlsText;

#[derive(Component)]
pub struct TipsElement;

pub fn spawn(mut commands: Commands, text_assets: Res<TextAssets>) {
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

    let text_style = TextStyle {
        font: text_assets.regular_font_handle.clone(),
        font_size: 14.0,
        color: Color::rgb(0.15, 0.15, 0.15),
    };

    let text_alignment = TextAlignment::TOP_LEFT;

    commands
        .spawn_bundle(
            Text2dBundle {
                text: Text::from_section(tips_text, text_style)
                    .with_alignment(text_alignment),
                transform: Transform::from_xyz(-140.0, 510.0, 1.0),
                ..default()
            },
            // AnimateScale,
        )
        .insert(TipsElement)
        .insert(ControlsText);
}
