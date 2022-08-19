use crate::domain::go_board::{GoPlayer, GoBoard};
use crate::domain::go_game::GoGame;

use super::place_piece_input::PlacePieceInput;
use super::place_piece_requester::PlacePieceRequester;
use super::place_piece_output::PlacePieceOutput;

impl PlacePieceRequester for GoGame {
    fn place_piece(&mut self, place_piece_input: PlacePieceInput) -> PlacePieceOutput {
        let mut board: &mut GoBoard = self.get_board();
        board.place(place_piece_input.row, place_piece_input.col, &place_piece_input.player);

        PlacePieceOutput {
            success: true,
            board_state: Some(board.get_board_state().clone()),
            next_player: Some(GoPlayer::WHITE),
            error_msg: None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::go_board::GoPlayer;

    use super::*;

    #[test]
    fn test_place_black_piece() {
        let mut game = GoGame::new(19);
        let input = PlacePieceInput {
            row: 0,
            col: 0,
            player: GoPlayer::BLACK
        };

        let output = game.place_piece(input);

        let mut expected_board_state = game.get_board().get_board_state().clone();
        expected_board_state[0][0] = Some(GoPlayer::BLACK);

        let expected_output = PlacePieceOutput {
            success: true,
            board_state: Some(expected_board_state),
            next_player: Some(GoPlayer::WHITE),
            error_msg: None
        };

        assert_eq!(expected_output, output);
    }
}