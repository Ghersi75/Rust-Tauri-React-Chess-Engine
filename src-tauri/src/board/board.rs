mod bitboard;

enum ActiveColor {
  White = 0,
  Black = 1,
}

pub enum BoardStateError {
  InvalidFENInput
}

impl ActiveColor {
  // Basic function for toggling between current ActiveColor
  pub fn toggle(self) -> Self {
    match self {
      ActiveColor::White => ActiveColor::Black,
      ActiveColor::Black => ActiveColor::White
    }
  }
}

// The board state will be stored as a FEN position so its easy to encode/decode
struct BoardState {
  // The boards will be stored as follows
  // --------------------------
  // Black Pieces
  // idx 0: p - Black Pawns
  // idx 1: n - Black Knights
  // idx 2: b - Black Bishop
  // idx 3: r - Black Rooks
  // idx 4: q - Black Queens
  // idx 5: k - Black King
  // --------------------------
  // White Pieces
  // idx 6: P - White Pawns
  // idx 7: N - White Knights
  // idx 8: B - White Bishop
  // idx 9: T - White Rooks
  // idx 10: Q - White Queens
  // idx 11: K - White King
  // --------------------------
  // Full Board
  // idx 12: All occupied squares
  bitboards: [bitboard::Bitboard; 13],
  // This keeps track of whose turn it is
  // Since White starts, it's defined as 0
  active_color: ActiveColor,
  // TODO: Come back and figure out castling
  castling: u8,
  // TODO: Come back and figure out en passant
  en_passant: u8,
  // This will keep track of halfmoves where no pawn was moved by either side and no pieces were capture
  // Very rare draw condition
  // TODO: Don't forget the logic for this
  halfmoves: u8,
  // This simply keeps track of the number of full moves so far
  // A full move is completed once white and black both moved a piece
  // a half move, also called a ply, is when either side completes a move
  fullmoves: u8
}

impl BoardState {
  pub fn init_from_fen(self) -> Result<Self, BoardStateError> {

  }
}