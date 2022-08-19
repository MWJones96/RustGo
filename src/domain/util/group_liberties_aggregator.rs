use std::collections::HashMap;
use std::collections::HashSet;

use super::super::go_board::GoBoard;
use super::super::go_board::GoPlayer;

pub type Liberties = HashSet<(u32, u32)>;
pub type Group = HashSet<(u32, u32)>;
pub type BlackGroups = Vec<Group>;
pub type WhiteGroups = Vec<Group>;

pub struct GroupLibertiesAggregator;

impl GroupLibertiesAggregator {
    pub fn get_group_liberties(board: &GoBoard) -> HashMap<GoPlayer, Vec<(Group, Liberties)>> {
        let mut liberties =
            HashMap::from([(GoPlayer::BLACK, Vec::new()), (GoPlayer::WHITE, Vec::new())]);

        let groups = Self::get_piece_groups(board);

        Self::insert_liberties(groups.0, &GoPlayer::BLACK, board, &mut liberties);
        Self::insert_liberties(groups.1, &GoPlayer::WHITE, board, &mut liberties);

        liberties
    }

    fn get_piece_groups(board: &GoBoard) -> (BlackGroups, WhiteGroups) {
        let mut black_groups = BlackGroups::new();
        let mut white_groups = WhiteGroups::new();

        let mut cloned_board = board.clone();

        white_groups.append(&mut Self::get_groups(&mut cloned_board, &GoPlayer::WHITE));
        black_groups.append(&mut Self::get_groups(&mut cloned_board, &GoPlayer::BLACK));

        (black_groups, white_groups)
    }

    fn get_groups(board: &mut GoBoard, player: &GoPlayer) -> Vec<Group> {
        let mut groups = Vec::new();
        for row in 0..board.get_board_state().len() {
            for col in 0..board.get_board_state()[0].len() {
                if let Some(i) = board.get_board_state()[row as usize][col as usize] {
                    match i == player.to_owned() {
                        true => {
                            let mut group = Group::new();
                            Self::get_group(board, &player, row as i32, col as i32, &mut group);

                            groups.push(group);
                        }
                        _ => continue,
                    }
                }
            }
        }

        groups
    }

    fn get_group(
        board: &mut GoBoard,
        player: &GoPlayer,
        row: i32,
        col: i32,
        current_group: &mut Group,
    ) {
        if row < 0
            || row >= board.get_board_state().len() as i32
            || col < 0
            || col >= board.get_board_state()[0].len() as i32
        {
            return;
        }

        if let Some(i) = board.get_board_state()[row as usize][col as usize] {
            match i == player.to_owned() {
                true => {
                    current_group.insert((row as u32, col as u32));
                    board.remove(row as u32, col as u32);

                    Self::get_group(board, player, row + 1, col, current_group);
                    Self::get_group(board, player, row - 1, col, current_group);
                    Self::get_group(board, player, row, col + 1, current_group);
                    Self::get_group(board, player, row, col - 1, current_group);
                }
                false => return,
            }
        }
    }

    fn insert_liberties(
        groups: Vec<Group>,
        player: &GoPlayer,
        board: &GoBoard,
        liberties: &mut HashMap<GoPlayer, Vec<(Group, Liberties)>>,
    ) {
        for group in groups {
            let mut group_liberties = Liberties::new();
            for (row, col) in group.iter() {
                let row = row.to_owned();
                let col = col.to_owned();

                if row > 0 && board.get_board_state()[(row - 1) as usize][col as usize].is_none() {
                    group_liberties.insert((row - 1, col));
                }
                if (row < (board.get_board_state().len() - 1) as u32
                    && board.get_board_state()[(row + 1) as usize][col as usize].is_none())
                {
                    group_liberties.insert((row + 1, col));
                }
                if col > 0 && board.get_board_state()[row as usize][(col - 1) as usize].is_none() {
                    group_liberties.insert((row, col - 1));
                }
                if (col < (board.get_board_state()[0].len() - 1) as u32
                    && board.get_board_state()[row as usize][(col + 1) as usize].is_none())
                {
                    group_liberties.insert((row, col + 1));
                }
            }
            let mut list = liberties.get(player).unwrap().to_owned();
            list.push((group, group_liberties));
            liberties.insert(player.to_owned(), list);
        }
    }
}

#[cfg(test)]
mod group_liberties_aggregator_tests {
    use super::*;

    #[test]
    fn test_liberties_white_groups() {
        let mut board = GoBoard::new(3);

        board.place(0, 0, &GoPlayer::WHITE);
        board.place(2, 2, &GoPlayer::WHITE);

        //|O| | |
        //| | | |
        //| | |O|

        let liberties = GroupLibertiesAggregator::get_group_liberties(&board);

        assert!(liberties.contains_key(&GoPlayer::BLACK));
        assert!(liberties.contains_key(&GoPlayer::WHITE));

        let black_liberties = liberties.get(&GoPlayer::BLACK).unwrap();
        let white_liberties = liberties.get(&GoPlayer::WHITE).unwrap();

        assert_eq!(&Vec::<(Group, Liberties)>::new(), black_liberties);
        assert_eq!(
            &vec![
                (Group::from([(0, 0)]), Liberties::from([(0, 1), (1, 0)])),
                (Group::from([(2, 2)]), Liberties::from([(1, 2), (2, 1)]))
            ],
            white_liberties
        );
    }

    #[test]
    fn test_liberties_black_group() {
        let mut board = GoBoard::new(3);

        board.place(0, 1, &GoPlayer::BLACK);

        board.place(1, 0, &GoPlayer::BLACK);
        board.place(1, 1, &GoPlayer::BLACK);
        board.place(1, 2, &GoPlayer::BLACK);

        board.place(2, 1, &GoPlayer::BLACK);

        //| |X| |
        //|X|X|X|
        //| |X| |

        let liberties = GroupLibertiesAggregator::get_group_liberties(&board);

        assert!(liberties.contains_key(&GoPlayer::BLACK));
        assert!(liberties.contains_key(&GoPlayer::WHITE));

        let black_liberties = liberties.get(&GoPlayer::BLACK).unwrap();
        let white_liberties = liberties.get(&GoPlayer::WHITE).unwrap();

        assert_eq!(
            &vec![(
                Group::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
                Liberties::from([(0, 0), (0, 2), (2, 0), (2, 2)])
            ),],
            black_liberties
        );
        assert_eq!(&Vec::<(Group, Liberties)>::new(), white_liberties);
    }

    #[test]
    fn test_liberties_black_and_white_groups() {
        let mut board = GoBoard::new(3);

        board.place(0, 1, &GoPlayer::BLACK);

        board.place(1, 0, &GoPlayer::BLACK);
        board.place(1, 1, &GoPlayer::BLACK);
        board.place(1, 2, &GoPlayer::BLACK);

        board.place(2, 1, &GoPlayer::BLACK);

        board.place(0, 0, &GoPlayer::WHITE);
        board.place(0, 2, &GoPlayer::WHITE);

        board.place(2, 0, &GoPlayer::WHITE);
        board.place(2, 2, &GoPlayer::WHITE);

        //|O|X|O|
        //|X|X|X|
        //|O|X|O|

        let liberties = GroupLibertiesAggregator::get_group_liberties(&board);

        assert!(liberties.contains_key(&GoPlayer::BLACK));
        assert!(liberties.contains_key(&GoPlayer::WHITE));

        let black_liberties = liberties.get(&GoPlayer::BLACK).unwrap();
        let white_liberties = liberties.get(&GoPlayer::WHITE).unwrap();

        assert_eq!(
            &vec![(
                Group::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
                Liberties::from([])
            )],
            black_liberties
        );

        assert_eq!(
            &vec![
                (Group::from([(0, 0)]), Liberties::from([])),
                (Group::from([(0, 2)]), Liberties::from([])),
                (Group::from([(2, 0)]), Liberties::from([])),
                (Group::from([(2, 2)]), Liberties::from([]))
            ],
            white_liberties
        );
    }
}

#[cfg(test)]
mod group_aggregator_tests {
    use super::*;

    #[test]
    fn test_get_piece_groups_empty() {
        let board = GoBoard::new(10);
        let old_board = board.clone();

        let groups = super::GroupLibertiesAggregator::get_piece_groups(&board);

        let black_groups = groups.0;
        let white_groups = groups.1;

        assert_eq!(0, black_groups.len());
        assert_eq!(0, white_groups.len());

        assert_eq!(old_board, board);
    }

    #[test]
    fn test_get_piece_groups_two_white_groups() {
        let mut board = GoBoard::new(3);

        board.place(0, 0, &GoPlayer::WHITE);
        board.place(2, 2, &GoPlayer::WHITE);

        let old_board = board.clone();

        //|O| | |
        //| | | |
        //| | |O|

        let groups = super::GroupLibertiesAggregator::get_piece_groups(&board);

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

        board.place(0, 1, &GoPlayer::BLACK);

        board.place(1, 0, &GoPlayer::BLACK);
        board.place(1, 1, &GoPlayer::BLACK);
        board.place(1, 2, &GoPlayer::BLACK);

        board.place(2, 1, &GoPlayer::BLACK);

        let old_board = board.clone();

        //| |X| |
        //|X|X|X|
        //| |X| |

        let groups = super::GroupLibertiesAggregator::get_piece_groups(&board);

        let black_groups = groups.0;
        let white_groups = groups.1;

        assert_eq!(1, black_groups.len());
        assert_eq!(0, white_groups.len());

        assert_eq!(
            HashSet::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
            black_groups[0]
        );

        assert_eq!(old_board, board);
    }

    #[test]
    fn test_get_piece_groups_black_and_white() {
        let mut board = GoBoard::new(3);

        board.place(0, 1, &GoPlayer::BLACK);

        board.place(1, 0, &GoPlayer::BLACK);
        board.place(1, 1, &GoPlayer::BLACK);
        board.place(1, 2, &GoPlayer::BLACK);

        board.place(2, 1, &GoPlayer::BLACK);

        board.place(0, 0, &GoPlayer::WHITE);
        board.place(0, 2, &GoPlayer::WHITE);

        board.place(2, 0, &GoPlayer::WHITE);
        board.place(2, 2, &GoPlayer::WHITE);

        let old_board = board.clone();

        //|O|X|O|
        //|X|X|X|
        //|O|X|O|

        let groups = super::GroupLibertiesAggregator::get_piece_groups(&board);

        let black_groups = groups.0;
        let white_groups = groups.1;

        assert_eq!(1, black_groups.len());
        assert_eq!(4, white_groups.len());

        assert_eq!(
            HashSet::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
            black_groups[0]
        );

        assert_eq!(HashSet::from([(0, 0)]), white_groups[0]);
        assert_eq!(HashSet::from([(0, 2)]), white_groups[1]);
        assert_eq!(HashSet::from([(2, 0)]), white_groups[2]);
        assert_eq!(HashSet::from([(2, 2)]), white_groups[3]);

        assert_eq!(old_board, board);
    }
}
