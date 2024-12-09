pub type Bitboard = u64;
pub enum BitBoardError {
  InvalidSquare
}

// Mutate in place
pub fn set_bit(bitboard: Bitboard, square: u8) -> Result<Bitboard, BitBoardError> {
  if square > 63 {
    return Err(BitBoardError::InvalidSquare);
  }
  // | is bitwise OR
  // anything OR 1 = 1
  // 1 | 1 = 1
  // 0 | 1 = 1
  Ok(bitboard | (1 << square))
}

// Clear a bit
pub fn clear_bit(bitboard: Bitboard, square: u8) -> Result<Bitboard, BitBoardError> {
  if square > 63 {
    return Err(BitBoardError::InvalidSquare);
  }

  // ! is bitwise NOT
  // !1 = 0
  // !0 = 1
  // & is bitwise AND
  // anything AND 0 = 0
  // 1 & 0 = 0
  // 0 & 0 = 0
  Ok(bitboard & !(1 << square))
}

pub fn toggle_bit(bitboard: Bitboard, square: u8) -> Result<Bitboard, BitBoardError> {
  if square > 63 {
    return Err(BitBoardError::InvalidSquare);
  }

  // ^ is bitwise XOR
  // 0 XOR 1 = 1
  // 1 XOR 1 = 0
  // current bit is toggled this way
  Ok(bitboard ^ (1 << square))
}

