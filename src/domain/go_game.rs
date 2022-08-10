use super::go_board::GoPlayer;
use super::go_board::GoBoard;
use super::group_aggregator::GroupAggregator;
use super::group_liberties_aggregator::GroupLibertiesAggregator;

pub struct GoGame {
    current_player: GoPlayer,
    board: GoBoard
}

impl GoGame {
    pub fn new(size: u32) -> Self {
        Self { 
            current_player: GoPlayer::BLACK,
            board: GoBoard::new(size)
        }
    }

    pub fn place(&mut self, row: u32, col: u32) -> bool {
        match self.board.place(row, col, self.current_player) {
            true => {
                let groups = GroupAggregator::get_piece_groups(&self.board);

                let remove_groups_with_no_liberties = |group| {
                    //Group with no remaining liberties
                    if GroupLibertiesAggregator::get_group_liberties(&self.board, group).len() == 0 {
                        group.iter().for_each(|point| self.board.remove(point.0, point.1));
                    }
                };

                match self.current_player {
                    GoPlayer::BLACK => {
                        groups.1.iter().for_each(remove_groups_with_no_liberties)
                    },
                    GoPlayer::WHITE => {
                        groups.0.iter().for_each(remove_groups_with_no_liberties)
                    },
                }

                self.current_player = match self.current_player {
                    GoPlayer::BLACK => GoPlayer::WHITE,
                    GoPlayer::WHITE => GoPlayer::BLACK
                };

                true
            },
            false => false
        }    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_black_piece() {
        let mut game = GoGame::new(19);
        
        assert_eq!(None, game.board.get_board_state()[0][0]);

        assert!(game.place(0, 0));

        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[0][0]);
        assert_eq!(None, game.board.get_board_state()[0][1]);

        assert_eq!(GoPlayer::WHITE, game.current_player);
    }

    #[test]
    fn test_place_black_and_white_pieces() {
        let mut game = GoGame::new(19);

        assert_eq!(None, game.board.get_board_state()[0][0]);
        assert_eq!(None, game.board.get_board_state()[0][1]);

        assert!(game.place(0, 0));
        assert!(game.place(0, 1));

        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[0][0]);
        assert_eq!(Some(GoPlayer::WHITE), game.board.get_board_state()[0][1]);

        assert_eq!(GoPlayer::BLACK, game.current_player);
    }

    #[test]
    fn test_place_same_place_twice() {
        let mut game = GoGame::new(19);

        assert_eq!(None, game.board.get_board_state()[0][0]);

        assert!(game.place(0, 0));
        assert!(!game.place(0, 0));

        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[0][0]);

        assert_eq!(GoPlayer::WHITE, game.current_player);
    }

    #[test]
    fn test_capture_black_group() {
        let mut game = GoGame::new(19);

        assert_eq!(None, game.board.get_board_state()[0][0]);

        assert!(game.place(0, 0)); //BLACK
        assert!(game.place(0, 1)); //WHITE
        assert!(game.place(5, 5)); //BLACK
        assert!(game.place(1, 0)); //WHITE

        assert_eq!(None, game.board.get_board_state()[0][0]);

        assert_eq!(Some(GoPlayer::WHITE), game.board.get_board_state()[0][1]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[5][5]);
        assert_eq!(Some(GoPlayer::WHITE), game.board.get_board_state()[1][0]);

        assert_eq!(GoPlayer::BLACK, game.current_player);
    }

    #[test]
    fn test_capture_white_group() {
        let mut game = GoGame::new(19);

        assert_eq!(None, game.board.get_board_state()[0][0]);

        assert!(game.place(0, 1)); //BLACK
        assert!(game.place(0, 0)); //WHITE
        assert!(game.place(1, 0)); //BLACK

        assert_eq!(None, game.board.get_board_state()[0][0]);

        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[0][1]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[1][0]);

        assert_eq!(GoPlayer::WHITE, game.current_player);
    }

    #[test]
    fn test_remove_larger_group() {
        let mut game = GoGame::new(3);

        assert!(game.place(0, 2)); //BLACK
        assert!(game.place(0, 0)); //WHITE
        assert!(game.place(1, 2)); //BLACK
        assert!(game.place(0, 1)); //WHITE
        assert!(game.place(2, 2)); //BLACK
        assert!(game.place(1, 0)); //WHITE
        assert!(game.place(2, 1)); //BLACK
        assert!(game.place(1, 1)); //WHITE
        assert!(game.place(2, 0)); //BLACK

        assert_eq!(None, game.board.get_board_state()[0][0]);
        assert_eq!(None, game.board.get_board_state()[0][1]);
        assert_eq!(None, game.board.get_board_state()[1][0]);
        assert_eq!(None, game.board.get_board_state()[1][1]);

        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[0][2]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[1][2]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[2][2]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[2][1]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[2][0]);

        assert_eq!(GoPlayer::WHITE, game.current_player);
    }

    #[test]
    fn test_not_remove_group_with_larger_board() {
        let mut game = GoGame::new(4);

        assert!(game.place(1, 3)); //BLACK
        assert!(game.place(1, 1)); //WHITE
        assert!(game.place(2, 3)); //BLACK
        assert!(game.place(1, 2)); //WHITE
        assert!(game.place(3, 3)); //BLACK
        assert!(game.place(2, 1)); //WHITE
        assert!(game.place(3, 2)); //BLACK
        assert!(game.place(2, 2)); //WHITE
        assert!(game.place(3, 1)); //BLACK

        assert_eq!(Some(GoPlayer::WHITE), game.board.get_board_state()[1][1]);
        assert_eq!(Some(GoPlayer::WHITE), game.board.get_board_state()[1][2]);
        assert_eq!(Some(GoPlayer::WHITE), game.board.get_board_state()[2][1]);
        assert_eq!(Some(GoPlayer::WHITE), game.board.get_board_state()[2][2]);

        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[1][3]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[2][3]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[3][3]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[3][2]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[3][1]);

        assert_eq!(GoPlayer::WHITE, game.current_player);
    }

    #[test]
    fn test_remove_two_groups_at_once() {
        let mut game = GoGame::new(3);

        assert!(game.place(1, 0)); //BLACK
        assert!(game.place(0, 0)); //WHITE
        assert!(game.place(1, 1)); //BLACK
        assert!(game.place(0, 2)); //WHITE
        assert!(game.place(1, 2)); //BLACK
        assert!(game.place(2, 0)); //WHITE
        assert!(game.place(0, 1)); //BLACK

        assert_eq!(None, game.board.get_board_state()[0][0]);
        assert_eq!(None, game.board.get_board_state()[0][2]);
        assert_eq!(Some(GoPlayer::WHITE), game.board.get_board_state()[2][0]);

        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[1][0]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[1][1]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[1][2]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[0][1]);

        assert_eq!(GoPlayer::WHITE, game.current_player);
    }

    #[test]
    fn test_remove_one_group_only() {
        let mut game = GoGame::new(4);

        assert!(game.place(1, 0)); //BLACK
        assert!(game.place(0, 0)); //WHITE
        assert!(game.place(1, 1)); //BLACK
        assert!(game.place(0, 2)); //WHITE
        assert!(game.place(1, 2)); //BLACK
        assert!(game.place(2, 0)); //WHITE
        assert!(game.place(0, 1)); //BLACK

        assert_eq!(None, game.board.get_board_state()[0][0]);

        assert_eq!(Some(GoPlayer::WHITE), game.board.get_board_state()[0][2]);
        assert_eq!(Some(GoPlayer::WHITE), game.board.get_board_state()[2][0]);

        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[1][0]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[1][1]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[1][2]);
        assert_eq!(Some(GoPlayer::BLACK), game.board.get_board_state()[0][1]);

        assert_eq!(GoPlayer::WHITE, game.current_player);
    }
}