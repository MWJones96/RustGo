use crate::domain::go_board::{GoPlayer};
use crate::domain::go_game::{GoGame, Snapshot};
use crate::domain::util::group_liberties_aggregator::GroupLibertiesAggregator;

use super::place_piece_input::PlacePieceInput;
use super::place_piece_requester::PlacePieceRequester;
use super::place_piece_output::PlacePieceOutput;

impl PlacePieceRequester for GoGame {
    fn place_piece(&mut self, place_piece_input: PlacePieceInput) -> PlacePieceOutput {
        let cloned_board_before_change = self.board.board_state.clone();

        if place_piece_input.player != self.current_player {
            return PlacePieceOutput {
                success: false,
                board_state: Some(cloned_board_before_change),
                next_player: Some(self.current_player),
                error_msg: Some(format!("The player {:?} tried to move when it is {:?}'s turn.", 
                    place_piece_input.player, self.current_player))
            };
        }

        let row = place_piece_input.row;
        let col = place_piece_input.col;

        let state_before_last = self.two_previous_states[1].as_ref();

        if self.has_violated_ko_rule(state_before_last, row, col) {
            return PlacePieceOutput {
                success: false,
                board_state: Some(cloned_board_before_change),
                next_player: Some(self.current_player),
                error_msg: Some(format!("The player {:?} has violated the Ko rule.", self.current_player))
            }
        }

        let result = self.board.place(row, col, &place_piece_input.player);

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

                self.two_previous_states.rotate_right(1);
                self.two_previous_states[0] = Some(Snapshot {
                    state: cloned_board_before_change,
                    chosen_move: (row, col),
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
                    board_state: Some(cloned_board_before_change),
                    next_player: Some(self.current_player),
                    error_msg: Some(format!(
                        "The player {:?} tried to place a piece on a square occupied by {:?}.",
                        place_piece_input.player, 
                        self.board.board_state[row as usize]
                            [col as usize].unwrap()
                    ))
                }
            }
        }
    }
}

impl GoGame {
    fn has_violated_ko_rule(&self, state_before_last: Option<&Snapshot>, row: u32, col: u32) -> bool {
        state_before_last.is_some() && 
        self.board.board_state == state_before_last.unwrap().state && 
        (row, col) == state_before_last.unwrap().chosen_move
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::go_board::{GoPlayer, GoBoard};

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

    #[test]
    fn test_violate_ko() {
        let mut game = GoGame::new(4);

        //| |O|X| |
        //|O|X| |X|
        //| |O|X| |
        //| | | | |

        game.place_piece(PlacePieceInput {
            row: 0,
            col: 2,
            player: GoPlayer::BLACK
        });
        game.place_piece(PlacePieceInput {
            row: 0,
            col: 1,
            player: GoPlayer::WHITE
        });
        game.place_piece(PlacePieceInput {
            row: 1,
            col: 1,
            player: GoPlayer::BLACK
        });
        game.place_piece(PlacePieceInput {
            row: 1,
            col: 0,
            player: GoPlayer::WHITE
        });
        game.place_piece(PlacePieceInput {
            row: 2,
            col: 2,
            player: GoPlayer::BLACK
        });
        game.place_piece(PlacePieceInput {
            row: 2,
            col: 1,
            player: GoPlayer::WHITE
        });
        game.place_piece(PlacePieceInput {
            row: 1,
            col: 3,
            player: GoPlayer::BLACK
        });

        let mut expected_board_state = GoBoard::new(4).board_state;
        expected_board_state[0][1] = Some(GoPlayer::WHITE);
        expected_board_state[1][0] = Some(GoPlayer::WHITE);
        expected_board_state[2][1] = Some(GoPlayer::WHITE);

        expected_board_state[0][2] = Some(GoPlayer::BLACK);
        expected_board_state[1][3] = Some(GoPlayer::BLACK);
        expected_board_state[2][2] = Some(GoPlayer::BLACK);
        expected_board_state[1][1] = Some(GoPlayer::BLACK);

        assert_eq!(expected_board_state, game.board.board_state);

        //White takes black
        game.place_piece(PlacePieceInput {
            row: 1,
            col: 2,
            player: GoPlayer::WHITE
        });

        //| |O|X| |
        //|O| |O|X|
        //| |O|X| |
        //| | | | |

        let mut expected_board_state = GoBoard::new(4).board_state;
        expected_board_state[0][1] = Some(GoPlayer::WHITE);
        expected_board_state[1][0] = Some(GoPlayer::WHITE);
        expected_board_state[2][1] = Some(GoPlayer::WHITE);
        expected_board_state[1][2] = Some(GoPlayer::WHITE);

        expected_board_state[0][2] = Some(GoPlayer::BLACK);
        expected_board_state[1][3] = Some(GoPlayer::BLACK);
        expected_board_state[2][2] = Some(GoPlayer::BLACK);

        assert_eq!(expected_board_state, game.board.board_state);

        //Black takes white
        game.place_piece(PlacePieceInput {
            row: 1,
            col: 1,
            player: GoPlayer::BLACK
        });

        //| |O|X| |
        //|O|X| |X|
        //| |O|X| |
        //| | | | |

        let mut expected_board_state = GoBoard::new(4).board_state;
        expected_board_state[0][1] = Some(GoPlayer::WHITE);
        expected_board_state[1][0] = Some(GoPlayer::WHITE);
        expected_board_state[2][1] = Some(GoPlayer::WHITE);

        expected_board_state[0][2] = Some(GoPlayer::BLACK);
        expected_board_state[1][3] = Some(GoPlayer::BLACK);
        expected_board_state[2][2] = Some(GoPlayer::BLACK);
        expected_board_state[1][1] = Some(GoPlayer::BLACK);

        assert_eq!(expected_board_state, game.board.board_state);

        //White violates ko rule

        let output = game.place_piece(PlacePieceInput {
            row: 1,
            col: 2,
            player: GoPlayer::WHITE
        });

        let expected_output = PlacePieceOutput {
            success: false,
            board_state: Some(expected_board_state),
            next_player: Some(GoPlayer::WHITE),
            error_msg: Some(String::from(
                "The player WHITE has violated the Ko rule."
            ))
        };

        assert_eq!(expected_output, output);

        let mut expected_board_state = GoBoard::new(4).board_state;
        expected_board_state[0][1] = Some(GoPlayer::WHITE);
        expected_board_state[1][0] = Some(GoPlayer::WHITE);
        expected_board_state[2][1] = Some(GoPlayer::WHITE);
        expected_board_state[3][3] = Some(GoPlayer::WHITE);

        expected_board_state[0][2] = Some(GoPlayer::BLACK);
        expected_board_state[1][3] = Some(GoPlayer::BLACK);
        expected_board_state[2][2] = Some(GoPlayer::BLACK);
        expected_board_state[1][1] = Some(GoPlayer::BLACK);

        //White moves legally
        let output = game.place_piece(PlacePieceInput {
            row: 3,
            col: 3,
            player: GoPlayer::WHITE
        });

        let expected_output = PlacePieceOutput {
            success: true,
            board_state: Some(expected_board_state),
            next_player: Some(GoPlayer::BLACK),
            error_msg: None
        };

        assert_eq!(expected_output, output);
    }
}