use std::collections::HashMap;

use bevy::{
    math::Vec2,
    prelude::{With, Without},
};

use crate::{
    fen::FenText,
    pieces::{ActivePiece, SelectedPiece, SourcePiece},
    tips::TipsText,
};

pub type Board = HashMap<&'static str, Vec2>;

pub type WithSelectedPiece = (
    With<SelectedPiece>,
    Without<SourcePiece>,
    Without<ActivePiece>,
);

pub type WithFenText = (With<FenText>, Without<TipsText>);

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
