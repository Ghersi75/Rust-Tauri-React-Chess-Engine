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

pub fn is_occupied(bitboard: Bitboard, square: u8) -> Result<bool, BitBoardError> {
  if square > 63 {
    return Err(BitBoardError::InvalidSquare);
  }

  // Create a mask with only the desired bit being 1
  let mask: Bitboard = 1 << square;
  // Anything & 1 = Anything
  // Anything & 0 = 0
  // Since only the bit we want to extract is has a 1 on the mask, the result will only keep track of that bit and clear the rest
  let res: Bitboard = bitboard & mask;

  // We want to shift the bit we care about so the resut is the right most bit
  // We return whether the value is 1, meaning the bit is occupied, or 0 meaning it's not occupied
  Ok(res >> square == 1)
}

