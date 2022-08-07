use crate::domain::go_board as go_board;
use std::collections::HashSet;
use std::collections::HashMap;

pub struct GroupAggregator;

impl GroupAggregator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_piece_groups(&self, board: &go_board::GoBoard) 
        -> HashMap<go_board::GoPlayer, Vec<HashSet<(u32, u32)>>> {
        let mut groups = HashMap::new();

        let mut cloned_board = board.clone();

        groups.insert(go_board::GoPlayer::WHITE, self.get_groups(
            &mut cloned_board, &go_board::GoPlayer::WHITE));
        groups.insert(go_board::GoPlayer::BLACK, self.get_groups(
            &mut cloned_board, &go_board::GoPlayer::BLACK));

        groups
    }

    fn get_groups(&self, board: &mut go_board::GoBoard, player: &go_board::GoPlayer) 
        -> Vec<HashSet<(u32, u32)>> {
        let mut groups = Vec::new();
        for row in 0..board.get_board_state().len() as i32 {
            for col in 0..board.get_board_state()[0].len() as i32 {
                if let Some(i) = board.get_board_state()[row as usize][col as usize] {
                    match i == player.to_owned() {
                        true => {
                            let mut group = HashSet::new();
                            self.get_group(board, &player, 
                                row, col, &mut group);
    
                            groups.push(group);
                        },
                        _ => continue,
                    }
                }
            }
        }

        groups
    }

    fn get_group(&self, board: &mut go_board::GoBoard, player: &go_board::GoPlayer, 
        row: i32, col: i32, current_group: &mut HashSet<(u32, u32)>) {
        if row < 0 || row >= board.get_board_state().len() as i32 
        || col < 0 || col >= board.get_board_state()[0].len() as i32 {
            return;
        }

        if let Some(i) = board.get_board_state()[row as usize][col as usize] {
            match i == player.to_owned() {
                true => {
                    current_group.insert((row as u32, col as u32));
                    board.remove(row, col);

                    self.get_group(board, player, row + 1, col, current_group);
                    self.get_group(board, player, row - 1 , col, current_group);
                    self.get_group(board, player, row, col + 1, current_group);
                    self.get_group(board, player, row, col - 1, current_group);
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
        let group_aggregator = GroupAggregator::new();
        let board = go_board::GoBoard::new(10);
        let old_board = board.clone();

        let groups = group_aggregator.get_piece_groups(&board);

        assert!(groups.contains_key(&go_board::GoPlayer::BLACK));
        assert!(groups.contains_key(&go_board::GoPlayer::WHITE));

        let black_groups = groups.get(&go_board::GoPlayer::BLACK).unwrap();
        let white_groups = groups.get(&go_board::GoPlayer::WHITE).unwrap();

        assert_eq!(0, black_groups.len());
        assert_eq!(0, white_groups.len());

        assert_eq!(old_board, board);
    }

    #[test]
    fn test_get_piece_groups_two_white_groups() {
        let mut board = go_board::GoBoard::new(3);

        board.place(0, 0, go_board::GoPlayer::WHITE);
        board.place(2, 2, go_board::GoPlayer::WHITE);

        let old_board = board.clone();

        //|O| | |
        //| | | |
        //| | |O|

        let group_aggregator = GroupAggregator::new();
        let groups = group_aggregator.get_piece_groups(&board);

        assert!(groups.contains_key(&go_board::GoPlayer::BLACK));
        assert!(groups.contains_key(&go_board::GoPlayer::WHITE));

        let black_groups = groups.get(&go_board::GoPlayer::BLACK).unwrap();
        let white_groups = groups.get(&go_board::GoPlayer::WHITE).unwrap();
    
        assert_eq!(0, black_groups.len());
        assert_eq!(2, white_groups.len());

        assert_eq!(HashSet::from([(0, 0)]), white_groups[0]);
        assert_eq!(HashSet::from([(2, 2)]), white_groups[1]);

        assert_eq!(old_board, board);
    }

    #[test]
    fn test_get_piece_groups_one_black_group() {
        let mut board = go_board::GoBoard::new(3);

        board.place(0, 1, go_board::GoPlayer::BLACK);

        board.place(1, 0, go_board::GoPlayer::BLACK);
        board.place(1, 1, go_board::GoPlayer::BLACK);
        board.place(1, 2, go_board::GoPlayer::BLACK);

        board.place(2, 1, go_board::GoPlayer::BLACK);

        let old_board = board.clone();

        //| |X| |
        //|X|X|X|
        //| |X| |

        let group_aggregator = GroupAggregator::new();
        let groups = group_aggregator.get_piece_groups(&board);

        assert!(groups.contains_key(&go_board::GoPlayer::BLACK));
        assert!(groups.contains_key(&go_board::GoPlayer::WHITE));

        let black_groups = groups.get(&go_board::GoPlayer::BLACK).unwrap();
        let white_groups = groups.get(&go_board::GoPlayer::WHITE).unwrap();
    
        assert_eq!(1, black_groups.len());
        assert_eq!(0, white_groups.len());

        assert_eq!(HashSet::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]), black_groups[0]);

        assert_eq!(old_board, board);
    }

    #[test]
    fn test_get_piece_groups_black_and_white() {
        let mut board = go_board::GoBoard::new(3);

        board.place(0, 1, go_board::GoPlayer::BLACK);

        board.place(1, 0, go_board::GoPlayer::BLACK);
        board.place(1, 1, go_board::GoPlayer::BLACK);
        board.place(1, 2, go_board::GoPlayer::BLACK);

        board.place(2, 1, go_board::GoPlayer::BLACK);

        board.place(0, 0, go_board::GoPlayer::WHITE);
        board.place(0, 2, go_board::GoPlayer::WHITE);

        board.place(2, 0, go_board::GoPlayer::WHITE);
        board.place(2, 2, go_board::GoPlayer::WHITE);

        let old_board = board.clone();

        //|O|X|O|
        //|X|X|X|
        //|O|X|O|

        let group_aggregator = GroupAggregator::new();
        let groups = group_aggregator.get_piece_groups(&board);

        assert!(groups.contains_key(&go_board::GoPlayer::BLACK));
        assert!(groups.contains_key(&go_board::GoPlayer::WHITE));

        let black_groups = groups.get(&go_board::GoPlayer::BLACK).unwrap();
        let white_groups = groups.get(&go_board::GoPlayer::WHITE).unwrap();

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
