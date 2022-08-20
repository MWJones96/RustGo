use crate::domain::go_board::{GoPlayer, GoBoard, GoBoardState};
use crate::domain::go_game::GoGame;
use crate::domain::util::group_liberties_aggregator::GroupLibertiesAggregator;

use super::place_piece_input::PlacePieceInput;
use super::place_piece_requester::PlacePieceRequester;
use super::place_piece_output::PlacePieceOutput;

impl PlacePieceRequester for GoGame {
    fn place_piece(&mut self, place_piece_input: PlacePieceInput) -> PlacePieceOutput {
        if place_piece_input.player != self.current_player {
            return PlacePieceOutput {
                success: false,
                board_state: Some(self.board.board_state.clone()),
                next_player: Some(self.current_player),
                error_msg: Some(format!("The player {:?} tried to move when it is {:?}'s turn.", 
                    place_piece_input.player, self.current_player))
            };
        }

        let result = self.board.place(place_piece_input.row, place_piece_input.col, &place_piece_input.player);

        match result {
            true => {
                match self.current_player {
                    GoPlayer::BLACK => self.current_player = GoPlayer::WHITE,
                    GoPlayer::WHITE => self.current_player = GoPlayer::BLACK
                }

                let group_liberties = GroupLibertiesAggregator::get_group_liberties(&self.board);
                let enemy_groups = group_liberties.get(&self.current_player).unwrap();

                enemy_groups.iter().for_each(|group| {
                    if group.1.len() == 0 {
                        for square in group.0.iter() {
                            self.board.remove(square.0, square.1)
                        }
                    }
                });

                PlacePieceOutput {
                    success: true,
                    board_state: Some(self.board.board_state.clone()),
                    next_player: Some(self.current_player),
                    error_msg: None
                }
            },
            false => {
                PlacePieceOutput {
                    success: false,
                    board_state: Some(self.board.board_state.clone()),
                    next_player: Some(self.current_player),
                    error_msg: Some(format!(
                        "The player {:?} tried to place a piece on a square occupied by {:?}.",
                        place_piece_input.player, 
                        self.board.board_state[place_piece_input.row as usize]
                            [place_piece_input.col as usize].unwrap()
                    ))
                }
            }
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

        let mut expected_board_state = GoBoard::new(19).board_state;
        expected_board_state[0][0] = Some(GoPlayer::BLACK);

        let expected_output = PlacePieceOutput {
            success: true,
            board_state: Some(expected_board_state),
            next_player: Some(GoPlayer::WHITE),
            error_msg: None
        };

        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_place_piece_wrong_player() {
        let mut game = GoGame::new(19);
        let input = PlacePieceInput {
            row: 0,
            col: 0,
            player: GoPlayer::WHITE
        };

        let output = game.place_piece(input);

        let expected_board_state = GoBoard::new(19).board_state;

        let expected_output = PlacePieceOutput {
            success: false,
            board_state: Some(expected_board_state),
            next_player: Some(GoPlayer::BLACK),
            error_msg: Some(String::from("The player WHITE tried to move when it is BLACK's turn."))
        };

        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_place_piece_already_there() {
        let mut game = GoGame::new(19);
        let input = PlacePieceInput {
            row: 0,
            col: 0,
            player: GoPlayer::BLACK
        };

        let output = game.place_piece(input);

        let mut expected_board_state = GoBoard::new(19).board_state;
        expected_board_state[0][0] = Some(GoPlayer::BLACK);

        let expected_output = PlacePieceOutput {
            success: true,
            board_state: Some(expected_board_state),
            next_player: Some(GoPlayer::WHITE),
            error_msg: None
        };

        assert_eq!(expected_output, output);

        let input = PlacePieceInput {
            row: 0,
            col: 0,
            player: GoPlayer::WHITE
        };

        let output = game.place_piece(input);

        let expected_output = PlacePieceOutput {
            success: false,
            board_state: Some(game.board.board_state.clone()),
            next_player: Some(GoPlayer::WHITE),
            error_msg: Some(String::from(
                "The player WHITE tried to place a piece on a square occupied by BLACK."
            ))
        };

        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_take_white_group() {
        let mut game = GoGame::new(2);
        game.place_piece(PlacePieceInput {
            row: 0,
            col: 1,
            player: GoPlayer::BLACK
        });
        game.place_piece(PlacePieceInput {
            row: 0,
            col: 0,
            player: GoPlayer::WHITE
        });
        let output = game.place_piece(PlacePieceInput {
            row: 1,
            col: 0,
            player: GoPlayer::BLACK
        });

        let mut expected_board_state = GoBoard::new(2).board_state;
        expected_board_state[0][1] = Some(GoPlayer::BLACK);
        expected_board_state[1][0] = Some(GoPlayer::BLACK);

        let expected_output = PlacePieceOutput {
            success: true,
            board_state: Some(expected_board_state),
            next_player: Some(GoPlayer::WHITE),
            error_msg: None
        };

        assert_eq!(expected_output, output);
    }
}