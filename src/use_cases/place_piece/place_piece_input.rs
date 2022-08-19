use crate::domain::go_board::GoPlayer;

pub struct PlacePieceInput {
    pub row: u32,
    pub col: u32,
    pub player: GoPlayer
}