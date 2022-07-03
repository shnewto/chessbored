#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ChessState {
    Setup,
    LoadingBoard,
    LoadingPieces,
    Loaded,
}
