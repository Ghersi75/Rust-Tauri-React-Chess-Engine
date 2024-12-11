use crate::board::board::ActiveColor;

enum PromotionType {
  Queen,
  Rook,
  Bishop,
  Knight
}

enum CapturedPiece {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen
}

enum Piece {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King
}

struct Move {
  // Starting square. Stored as a u8 between 0-63 inclusive. Bitboard operations are easy if we know which square
  from_square: u8,
  // Ending square. Same as above. Stored as u8 between 0-63 inclusive
  to_square: u8,
  // What piece type was moved
  piece_moved: Piece,
  // Which color was active at the time of the move. Represents who made the move
  active_color: ActiveColor,
  // Keeps track of whether a pawn promoted and to what piece
  promotion: Option<PromotionType>,
  // Keeps track of whether a piece was captured and what piece of so
  captured_piece: Option<CapturedPiece>
}

impl Move {
  
}