use std::u8;

use crate::board::bitboard;
use regex::Regex;

use super::bitboard::Bitboard;

enum ActiveColor {
  White = 0,
  Black = 1,
}

pub enum BoardStateError {
  InvalidFENInput,
  BitBoardOperationError,
  InvalidActiveColor,
  CharConversionError
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
  //  -> Result<Self, BoardStateError>
  pub fn init_from_fen(fen: &str) -> Result<Self, BoardStateError> {
    // https://chess.stackexchange.com/a/1487
    // Going to trust this random hero and assume this is valid regex to check fen notation
    let re = Regex::new(r"\s*([rnbqkpRNBQKP1-8]+\/){7}([rnbqkpRNBQKP1-8]+)\s[bw-]\s(([a-hkqA-HKQ]{1,4})|(-))\s(([a-h][36])|(-))\s\d+\s\d+\s*").unwrap();
    if !re.is_match(fen) {
      return Err(BoardStateError::InvalidFENInput);
    }

    let mut blk_p: Bitboard = 0;
    let mut blk_n: Bitboard = 0;
    let mut blk_b: Bitboard = 0;
    let mut blk_r: Bitboard = 0;
    let mut blk_q: Bitboard = 0;
    let mut blk_k: Bitboard = 0;

    let mut wht_p: Bitboard = 0;
    let mut wht_n: Bitboard = 0;
    let mut wht_b: Bitboard = 0;
    let mut wht_r: Bitboard = 0;
    let mut wht_q: Bitboard = 0;
    let mut wht_k: Bitboard = 0;

    let split_fen: Vec<&str> = fen.split(" ").collect();
    let fen_position: &str = split_fen[0];
    let (mut row, mut col): (u8, u8) = (0, 0);
    for i in 0..fen_position.len() {
      if let Some(curr_char) = fen_position.chars().nth(i) {
        match curr_char {
          '/' => {
            row += 1;
            col = 0;
          }
          '1'..='8' => col += u8::try_from(curr_char).map_err(|e| BoardStateError::CharConversionError)?,
          'p' => blk_p = bitboard::set_bit(blk_p, row * 8 + col).map_err(|e| BoardStateError::BitBoardOperationError)?,
          'n' => blk_n = bitboard::set_bit(blk_n, row * 8 + col).map_err(|e| BoardStateError::BitBoardOperationError)?,
          'b' => blk_b = bitboard::set_bit(blk_b, row * 8 + col).map_err(|e| BoardStateError::BitBoardOperationError)?,
          'r' => blk_r = bitboard::set_bit(blk_r, row * 8 + col).map_err(|e| BoardStateError::BitBoardOperationError)?,
          'q' => blk_q = bitboard::set_bit(blk_q, row * 8 + col).map_err(|e| BoardStateError::BitBoardOperationError)?,
          'k' => blk_k = bitboard::set_bit(blk_k, row * 8 + col).map_err(|e| BoardStateError::BitBoardOperationError)?,
          'p' => wht_p = bitboard::set_bit(blk_p, row * 8 + col).map_err(|e| BoardStateError::BitBoardOperationError)?,
          'n' => wht_n = bitboard::set_bit(blk_p, row * 8 + col).map_err(|e| BoardStateError::BitBoardOperationError)?,
          'b' => wht_b = bitboard::set_bit(blk_p, row * 8 + col).map_err(|e| BoardStateError::BitBoardOperationError)?,
          'r' => wht_r = bitboard::set_bit(blk_p, row * 8 + col).map_err(|e| BoardStateError::BitBoardOperationError)?,
          'q' => wht_q = bitboard::set_bit(blk_p, row * 8 + col).map_err(|e| BoardStateError::BitBoardOperationError)?,
          'k' => wht_k = bitboard::set_bit(blk_p, row * 8 + col).map_err(|e| BoardStateError::BitBoardOperationError)?,
          // Somehow a character wasn't covered, so it must be invalid
          // Unlikely since regex should check it, but I'm sure if that pattern works or not
          // Better safe than sorry
          _ => return Err(BoardStateError::InvalidFENInput)
        }
        col += 1;
      }
    }

    let mut active_color: ActiveColor;
    match split_fen[1] {
      "w" => active_color = ActiveColor::White,
      "b" => active_color = ActiveColor::Black,
      _ => return Err(BoardStateError::InvalidActiveColor)
    }

    // TODO: Fix castling logic
    let castling: u8 = 0;

    // TODO: Fix en passant logic
    let en_passant: u8 = 0;

    let halfmoves: u8 = split_fen[4].parse::<u8>().map_err(|e| BoardStateError::CharConversionError)?;
    let fullmoves: u8 = split_fen[5].parse::<u8>().map_err(|e| BoardStateError::CharConversionError)?;

    // OR would work here, but in case something was messed up, XOR will remove overlapping pieces and I can debug later if pieces are missing
    // It should not cause issues tho since no pieces should be overlapping
    let full_board: Bitboard = blk_p ^ blk_n ^ blk_b ^ blk_r ^ blk_q ^ blk_k ^ wht_p ^ wht_n ^ wht_b ^ wht_r ^ wht_q ^ wht_k;
    let bitboards: [Bitboard; 13] = [blk_p, blk_n, blk_b, blk_r, blk_q, blk_k, wht_p, wht_n, wht_b, wht_r, wht_q, wht_k, full_board];
    Ok(Self{
      bitboards,
      active_color,
      castling,
      en_passant,
      halfmoves,
      fullmoves
    })
  }

  pub fn init_default() -> Result<Self, BoardStateError> {
    // https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation#Examples
    let fen_starting_pos: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    Self::init_from_fen(fen_starting_pos)
  }
}