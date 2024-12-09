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
  from_square: u8,
  to_square: u8,
  piece_moved: Piece,
  promotion: Option<PromotionType>,
  captured_piece: Option<CapturedPiece>
}

impl Move {
  
}