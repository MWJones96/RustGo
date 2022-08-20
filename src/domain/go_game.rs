use super::go_board::GoPlayer;
use super::go_board::GoBoard;
use super::util::group_liberties_aggregator::Group;
use super::util::group_liberties_aggregator::GroupLibertiesAggregator;
use super::util::group_liberties_aggregator::Liberties;

use std::sync::Mutex;
use std::thread;

#[derive(Clone)]
pub struct GoGame {
    pub current_player: GoPlayer,
    pub board: GoBoard
}

impl GoGame {
    pub fn new(size: u32) -> Self {
        Self { 
            current_player: GoPlayer::BLACK,
            board: GoBoard::new(size)
        }
    }
}