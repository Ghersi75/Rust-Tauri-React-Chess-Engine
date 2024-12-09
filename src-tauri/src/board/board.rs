use core::fmt;
use std::{collections::HashMap, u8};

use crate::board::bitboard;
use regex::Regex;

use super::bitboard::{is_occupied, set_bit, BitBoardError, Bitboard};

pub enum ActiveColor {
  White = 0,
  Black = 1,
}

pub enum BoardStateError {
  InvalidFENInput,
  BitBoardOperationError,
  InvalidActiveColor,
  CharConversionError
}

impl fmt::Display for BoardStateError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::InvalidFENInput => write!(f, "Invalid FEN Input"),
      Self::BitBoardOperationError => write!(f, "BitBoard Operation Error"),
      Self::InvalidActiveColor => write!(f, "Invalid Active Color"),
      Self::CharConversionError => write!(f, "Char Conversion Error")
    }
  }
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
pub struct BoardState {
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

    let mut bitboards: [Bitboard; 13] = [0; 13];

    let piece_map: HashMap<char, usize> = HashMap::from([
      ('p', 0), // Black pawn
      ('n', 1), // Black knight
      ('b', 2), // Black bishop
      ('r', 3), // Black rook
      ('q', 4), // Black queen
      ('k', 5), // Black king
      ('P', 6), // White pawn
      ('N', 7), // White knight
      ('B', 8), // White bishop
      ('R', 9), // White rook
      ('Q', 10), // White queen
      ('K', 11), // White king
    ]);

    let split_fen: Vec<&str> = fen.split(" ").collect();
    let fen_position: &str = split_fen[0];
    let (mut row, mut col): (u8, u8) = (0, 0);
    for i in 0..fen_position.len() {
      if let Some(curr_char) = fen_position.chars().nth(i) {
        let curr_square = row * 8 + col;
        // println!("curr_char: {curr_char}, row: {row}, col: {col}, idx: {}", row * 8 + col);
        // TODO: Fix this ugly match statement
        match curr_char {
          '/' => {
            row += 1;
            col = 0;
          }
          '1'..='8' => col += curr_char.to_digit(10).ok_or(BoardStateError::CharConversionError)? as u8,
          _ => {
            if let Some(&piece_idx) = piece_map.get(&curr_char) {
              bitboards[piece_idx] = set_bit(bitboards[piece_idx], curr_square).map_err(|e| BoardStateError::BitBoardOperationError)?;
              col += 1;
            } else {
              return Err(BoardStateError::InvalidFENInput);
            }
          }
        }
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

    // Fold goes through all items and accumulates the result and returns it at the end
    // In this case, at each step it runs ^ (XOR) on acc with each new curr value
    // 0 is just the starting point, an empty bitbaord in this case
    // OR would work here, but in case something was messed up, XOR will remove overlapping pieces and I can debug later if pieces are missing
    // It should not cause issues tho since no pieces should be overlapping
    bitboards[12] = bitboards[0..12].iter().fold(0, |acc, curr| acc ^ curr);
    // Works as intended
    // println!("{:064b}", bitboards[12]);
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

  pub fn get_piece_at_square(&self, square: u8) -> Result<char, BoardStateError> {
    let piece_map = [
      ('p', 0), // Black pawn
      ('n', 1), // Black knight
      ('b', 2), // Black bishop
      ('r', 3), // Black rook
      ('q', 4), // Black queen
      ('k', 5), // Black king
      ('P', 6), // White pawn
      ('N', 7), // White knight
      ('B', 8), // White bishop
      ('R', 9), // White rook
      ('Q', 10), // White queen
      ('K', 11), // White king
    ];

    let mut res: char = '.';
    for (piece_char, piece_bitboard_idx) in piece_map.iter() {
      if is_occupied(self.bitboards[*piece_bitboard_idx], square).map_err(|e| BoardStateError::BitBoardOperationError)? {
        res = *piece_char;
      }
    }

    Ok(res)
  }

  pub fn print_board(self) {
    for row in 0..8 {
      for col in 0..8 {
        let curr_square = row * 8 + col;
        // Check that current square is occupied before printing
        let curr_piece: char = self.get_piece_at_square(curr_square).unwrap_or('.');
        print!("{} ", curr_piece);
      }
      println!();
    }
  }
}