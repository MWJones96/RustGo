use super::{place_piece_output::PlacePieceOutput, place_piece_input::PlacePieceInput};

pub trait PlacePieceRequester {
    fn place_piece(&mut self, place_piece_input: PlacePieceInput) -> PlacePieceOutput;
}