use std::collections::HashMap;

use bevy::prelude::Component;
use bevy::{
    math::Vec2,
    prelude::{With, Without},
};

use crate::{
    control_ux::ControlsText,
    fen::FenText,
    pieces::{ActivePiece, SelectedPiece, SourcePiece},
};

use shakmaty::Chess;

pub type Board = HashMap<&'static str, Vec2>;

#[derive(Clone, Debug, Default, Component)]
pub struct Moves {
    pub data: Chess,
    pub in_use: bool,
}

pub type WithSelectedPiece = (
    With<SelectedPiece>,
    Without<SourcePiece>,
    Without<ActivePiece>,
);

pub type WithFenText = (With<FenText>, Without<ControlsText>);

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
