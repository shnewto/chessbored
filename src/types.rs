use std::collections::HashMap;

use bevy::{
    hierarchy::Children,
    math::Vec2,
    prelude::{With, Without},
    ui::Interaction,
};

use crate::pieces::{ActivePiece, SelectedPiece, SourcePiece};

pub type Board = HashMap<&'static str, Vec2>;

pub type ButtonInteraction<'a> = (&'a Interaction, &'a Children);

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
