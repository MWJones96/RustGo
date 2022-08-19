use crate::domain::go_board::{GoBoardState, GoPlayer};

#[derive(PartialEq, Debug)]
pub struct PlacePieceOutput {
    pub success: bool,
    pub board_state: Option<GoBoardState>,
    pub next_player: Option<GoPlayer>,
    pub error_msg: Option<String>
}