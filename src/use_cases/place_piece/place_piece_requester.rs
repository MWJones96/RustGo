use super::place_piece_output::PlacePieceOutput;

pub trait PlacePieceRequester {
    fn place_piece(&self, row: u32, col: u32) -> PlacePieceOutput;
}