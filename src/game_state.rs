use crate::piece::*;

pub struct GameState {
    pub turn: Color,
    pub pieces: Vec<Piece>,
    pub selected_piece_index: Option<usize>,
    pub possible_positions: Vec<Position>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            turn: Color::WHITE,
            pieces: vec![
                Piece {
                    color: Color::WHITE,
                    piece_type: PieceType::ROOK,
                    pos: Position { x: 0, y: 7 },
                },
                Piece {
                    color: Color::WHITE,
                    piece_type: PieceType::KNIGHT,
                    pos: Position { x: 1, y: 7 },
                },
                Piece {
                    color: Color::WHITE,
                    piece_type: PieceType::BISHOP,
                    pos: Position { x: 2, y: 7 },
                },
            ],
            selected_piece_index: None,
            possible_positions: vec![],
        }
    }

    fn calculate_possible_positions() {}
}
