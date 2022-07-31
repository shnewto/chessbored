#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ChessState {
    Setup,
    Loading,
    Loaded,
    Running,
}
