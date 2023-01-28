pub enum Color {
    BLACK,
    WHITE,
}

pub enum PieceType {
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    QUEEN,
    KING,
}

pub struct Position {
    pub x: u8,
    pub y: u8,
}

pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub pos: Position,
}

impl Piece {
    pub fn move_pos(&mut self, pos: Position) {
        self.pos = pos;
    }
}
