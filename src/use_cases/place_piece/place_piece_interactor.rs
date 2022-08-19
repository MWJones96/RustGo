use crate::domain::go_game::GoGame;

use super::place_piece_requester::PlacePieceRequester;
use super::place_piece_output::PlacePieceOutput;

impl PlacePieceRequester for GoGame {
    fn place_piece(&self, row: u32, col: u32) -> PlacePieceOutput {
        


        PlacePieceOutput
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy() {
    }
}