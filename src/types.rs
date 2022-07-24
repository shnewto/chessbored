use std::collections::HashMap;

use bevy::{
    hierarchy::Children,
    math::Vec2,
    prelude::{Component, With, Without},
    ui::Interaction,
};

use crate::pieces::{ActivePiece, SelectedPiece, SourcePiece};

pub type Board = HashMap<&'static str, Vec2>;

pub type ButtonInteraction<'a> = (&'a Interaction, &'a Children);

#[derive(Component, Debug, Default)]
pub struct BoardAssets {
    pub dark_square: String,
    pub light_square: String,
}

pub type WithSelectedPiece = (
    With<SelectedPiece>,
    Without<SourcePiece>,
    Without<ActivePiece>,
);

pub type WithActivePiece = (
    With<ActivePiece>,
    Without<SourcePiece>,
    Without<SelectedPiece>,
);

pub type WithSourcePiece = (
    With<SourcePiece>,
    Without<ActivePiece>,
    Without<SelectedPiece>,
);
