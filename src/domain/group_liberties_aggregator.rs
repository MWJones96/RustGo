use std::collections::HashMap;
use std::collections::HashSet;

use super::go_board::GoBoard;
use super::go_board::GoPlayer;

use super::group_aggregator::GroupAggregator;

use crate::domain::group_aggregator::Group as Group;
pub type Liberties = HashSet<(u32, u32)>;

pub struct GroupLibertiesAggregator;

impl GroupLibertiesAggregator {
    pub fn get_group_liberties(board: &GoBoard, group: &Group) -> Liberties {
        let groups = GroupAggregator::get_piece_groups(board);

        let black_groups = groups.0;
        let white_groups = groups.1;

        if !black_groups.contains(group) && !white_groups.contains(group) {
            panic!("Group input is invalid");
        }

        let mut liberties = Liberties::new();
        for (row, col) in group.iter() {
            let row = row.to_owned();
            let col = col.to_owned();

            if row > 0 && board.get_board_state()[(row - 1) as usize][col as usize].is_none() {
                liberties.insert((row - 1, col));
            }
            if (row < (board.get_board_state().len() - 1) as u32
                && board.get_board_state()[(row + 1) as usize][col as usize].is_none()) {
                liberties.insert((row + 1, col));
            }
            if col > 0 && board.get_board_state()[row as usize][(col - 1) as usize].is_none() {
                liberties.insert((row, col - 1));
            }
            if (col < (board.get_board_state()[0].len() - 1) as u32
                && board.get_board_state()[row as usize][(col + 1) as usize].is_none()) {
                liberties.insert((row, col + 1));
            }
        }

        liberties
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liberties_white_groups() {
        let mut board = GoBoard::new(3);

        board.place(0, 0, GoPlayer::WHITE);
        board.place(2, 2, GoPlayer::WHITE);

        //|O| | |
        //| | | |
        //| | |O|

        let white_liberties_group_one = 
            GroupLibertiesAggregator::get_group_liberties(
                &board, &Group::from([(0, 0)]));
        let white_liberties_group_two = 
            GroupLibertiesAggregator::get_group_liberties(
                &board, &Group::from([(2, 2)]));

        assert_eq!(Liberties::from([(0, 1), (1, 0)]), white_liberties_group_one);
        assert_eq!(Liberties::from([(1, 2), (2, 1)]), white_liberties_group_two);
    }

    #[test]
    fn test_liberties_black_group() {
        let mut board = GoBoard::new(3);

        board.place(0, 1, GoPlayer::BLACK);

        board.place(1, 0, GoPlayer::BLACK);
        board.place(1, 1, GoPlayer::BLACK);
        board.place(1, 2, GoPlayer::BLACK);

        board.place(2, 1, GoPlayer::BLACK);

        //| |X| |
        //|X|X|X|
        //| |X| |

        let black_liberties = 
            GroupLibertiesAggregator::get_group_liberties(
                &board, &Group::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]));

        assert_eq!(Liberties::from([(0, 0), (0, 2), (2, 0), (2, 2)]), black_liberties);
    }

    #[test]
    fn test_liberties_black_and_white_groups() {
        let mut board = GoBoard::new(3);

        board.place(0, 1, GoPlayer::BLACK);

        board.place(1, 0, GoPlayer::BLACK);
        board.place(1, 1, GoPlayer::BLACK);
        board.place(1, 2, GoPlayer::BLACK);

        board.place(2, 1, GoPlayer::BLACK);

        board.place(0, 0, GoPlayer::WHITE);
        board.place(0, 2, GoPlayer::WHITE);

        board.place(2, 0, GoPlayer::WHITE);
        board.place(2, 2, GoPlayer::WHITE);

        //|O|X|O|
        //|X|X|X|
        //|O|X|O|

        let black_liberties = 
            GroupLibertiesAggregator::get_group_liberties(
                &board, &Group::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]));

        assert_eq!(Liberties::from([]), black_liberties);

        let white_liberties = 
            GroupLibertiesAggregator::get_group_liberties(
                &board, &Group::from([(0, 0)]));

        assert_eq!(Liberties::from([]), white_liberties);

        let white_liberties = 
            GroupLibertiesAggregator::get_group_liberties(
                &board, &Group::from([(0, 2)]));

        assert_eq!(Liberties::from([]), white_liberties);

        let white_liberties = 
            GroupLibertiesAggregator::get_group_liberties(
                &board, &Group::from([(2, 0)]));

        assert_eq!(Liberties::from([]), white_liberties);

        let white_liberties = 
            GroupLibertiesAggregator::get_group_liberties(
                &board, &Group::from([(2, 2)]));

        assert_eq!(Liberties::from([]), white_liberties);
    }

    #[test]
    #[should_panic]
    fn test_panic_on_invalid_group_input() {
        let mut board = GoBoard::new(3);

        GroupLibertiesAggregator::get_group_liberties(
            &board, &Group::from([(0, 0)]));
    }
}