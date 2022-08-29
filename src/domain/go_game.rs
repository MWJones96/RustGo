use super::go_board::GoBoardState;
use super::go_board::GoPlayer;
use super::go_board::GoBoard;
use super::util::group_liberties_aggregator::Group;
use super::util::group_liberties_aggregator::GroupLibertiesAggregator;
use super::util::group_liberties_aggregator::Liberties;

use std::sync::Mutex;
use std::thread;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Snapshot {
    pub state: GoBoardState,
    pub chosen_move: (u32, u32)
}

#[derive(Clone)]
pub struct GoGame {
    pub current_player: GoPlayer,
    pub board: GoBoard,
    pub two_previous_states: Vec<Option<Snapshot>>
}

impl GoGame {
    pub fn new(size: u32) -> Self {
        Self { 
            current_player: GoPlayer::BLACK,
            board: GoBoard::new(size),
            two_previous_states: vec![None, None]
        }
    }
}