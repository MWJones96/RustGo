use super::go_board::GoBoard;
use super::go_board::GoPlayer;

use std::collections::HashSet;

pub type Group = HashSet<(u32, u32)>;
pub type BlackGroups = Vec<Group>;
pub type WhiteGroups = Vec<Group>;
pub struct GroupAggregator;

impl GroupAggregator {
    pub fn get_piece_groups(board: &GoBoard) 
            -> (BlackGroups, WhiteGroups) {
        let mut black_groups = BlackGroups::new();
        let mut white_groups = WhiteGroups::new();

        let mut cloned_board = board.clone();

        white_groups.append(&mut Self::get_groups(
            &mut cloned_board, &GoPlayer::WHITE));
        black_groups.append(&mut Self::get_groups(
            &mut cloned_board, &GoPlayer::BLACK));

        (black_groups, white_groups)
    }

    fn get_groups(board: &mut GoBoard, player: &GoPlayer) 
        -> Vec<Group> {
        let mut groups = Vec::new();
        for row in 0..board.get_board_state().len() {
            for col in 0..board.get_board_state()[0].len() {
                if let Some(i) = board.get_board_state()[row as usize][col as usize] {
                    match i == player.to_owned() {
                        true => {
                            let mut group = Group::new();
                            Self::get_group(board, &player, 
                                row as i32, col as i32, &mut group);
    
                            groups.push(group);
                        },
                        _ => continue,
                    }
                }
            }
        }

        groups
    }

    fn get_group(board: &mut GoBoard, player: &GoPlayer, 
        row: i32, col: i32, current_group: &mut Group) {
        if row < 0 || row >= board.get_board_state().len() as i32 
        || col < 0 || col >= board.get_board_state()[0].len() as i32 {
            return;
        }

        if let Some(i) = board.get_board_state()[row as usize][col as usize] {
            match i == player.to_owned() {
                true => {
                    current_group.insert((row as u32, col as u32));
                    board.remove(row as u32, col as u32);

                    Self::get_group(board, player, row + 1, col, current_group);
                    Self::get_group(board, player, row - 1 , col, current_group);
                    Self::get_group(board, player, row, col + 1, current_group);
                    Self::get_group(board, player, row, col - 1, current_group);
                },
                false => return,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_piece_groups_empty() {
        let board = GoBoard::new(10);
        let old_board = board.clone();

        let groups = GroupAggregator::get_piece_groups(&board);

        let black_groups = groups.0;
        let white_groups = groups.1;

        assert_eq!(0, black_groups.len());
        assert_eq!(0, white_groups.len());

        assert_eq!(old_board, board);
    }

    #[test]
    fn test_get_piece_groups_two_white_groups() {
        let mut board = GoBoard::new(3);

        board.place(0, 0, GoPlayer::WHITE);
        board.place(2, 2, GoPlayer::WHITE);

        let old_board = board.clone();

        //|O| | |
        //| | | |
        //| | |O|

        let groups = GroupAggregator::get_piece_groups(&board);

        let black_groups = groups.0;
        let white_groups = groups.1;
    
        assert_eq!(0, black_groups.len());
        assert_eq!(2, white_groups.len());

        assert_eq!(HashSet::from([(0, 0)]), white_groups[0]);
        assert_eq!(HashSet::from([(2, 2)]), white_groups[1]);

        assert_eq!(old_board, board);
    }

    #[test]
    fn test_get_piece_groups_one_black_group() {
        let mut board = GoBoard::new(3);

        board.place(0, 1, GoPlayer::BLACK);

        board.place(1, 0, GoPlayer::BLACK);
        board.place(1, 1, GoPlayer::BLACK);
        board.place(1, 2, GoPlayer::BLACK);

        board.place(2, 1, GoPlayer::BLACK);

        let old_board = board.clone();

        //| |X| |
        //|X|X|X|
        //| |X| |

        let groups = GroupAggregator::get_piece_groups(&board);

        let black_groups = groups.0;
        let white_groups = groups.1;
    
        assert_eq!(1, black_groups.len());
        assert_eq!(0, white_groups.len());

        assert_eq!(HashSet::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]), black_groups[0]);

        assert_eq!(old_board, board);
    }

    #[test]
    fn test_get_piece_groups_black_and_white() {
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

        let old_board = board.clone();

        //|O|X|O|
        //|X|X|X|
        //|O|X|O|

        let groups = GroupAggregator::get_piece_groups(&board);

        let black_groups = groups.0;
        let white_groups = groups.1;

        assert_eq!(1, black_groups.len());
        assert_eq!(4, white_groups.len());

        assert_eq!(HashSet::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]), black_groups[0]);

        assert_eq!(HashSet::from([(0, 0)]), white_groups[0]);
        assert_eq!(HashSet::from([(0, 2)]), white_groups[1]);
        assert_eq!(HashSet::from([(2, 0)]), white_groups[2]);
        assert_eq!(HashSet::from([(2, 2)]), white_groups[3]);

        assert_eq!(old_board, board);
    }
}
