pub type GoBoardState = Vec<Vec<Option<GoPlayer>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GoPlayer{
    BLACK,
    WHITE
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoBoard{
    board_state: GoBoardState
}

impl GoBoard {
    pub fn new(size: u32) -> Self {
        Self {
            board_state: 
                vec![vec![None; size as usize]; size as usize]
        }
    }

    pub fn place(&mut self, row: u32, col: u32, piece: GoPlayer) -> bool {
        match self.board_state[row as usize][col as usize] {
            Some(_) => false,
            None => {
                self.board_state[row as usize][col as usize] = Some(piece);
                true
            }
        }
    }

    pub fn remove(&mut self, row: u32, col: u32) {
        self.board_state[row as usize][col as usize] = None;
    }

    pub fn get_board_state(&self) -> &GoBoardState { 
        return &self.board_state; 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = GoBoard::new(10);
        assert_eq!(10, board.get_board_state().len());
        assert_eq!(10, board.get_board_state()[0].len());
    }

    #[test]
    fn test_place_pieces() {
        let mut board = GoBoard::new(10);

        assert!(board.place(0, 0, GoPlayer::BLACK));

        assert!(!board.place(0, 0, GoPlayer::BLACK));
        assert!(!board.place(0, 0, GoPlayer::WHITE));

        assert_eq!(Some(GoPlayer::BLACK), board.get_board_state()[0][0]);

        assert!(board.place(5, 5, GoPlayer::BLACK));
        assert!(board.place(9, 9, GoPlayer::WHITE));

        assert_eq!(Some(GoPlayer::BLACK), board.get_board_state()[5][5]);
        assert_eq!(Some(GoPlayer::WHITE), board.get_board_state()[9][9]);

    }

    #[test]
    fn test_remove_pieces() {
        let mut board = GoBoard::new(10);

        assert_eq!(None, board.get_board_state()[0][0]);
        board.remove(0, 0);
        assert_eq!(None, board.get_board_state()[0][0]);

        assert_eq!(None, board.get_board_state()[5][5]);
        board.place(5, 5, GoPlayer::BLACK);
        assert_eq!(Some(GoPlayer::BLACK), board.get_board_state()[5][5]);
        board.remove(5, 5);
        assert_eq!(None, board.get_board_state()[5][5]);
    }
}


