import blk_pawn from "../assets/blk_pawn.svg"
import blk_knight from "../assets/blk_knight.svg"
import blk_bishop from "../assets/blk_bishop.svg"
import blk_rook from "../assets/blk_rook.svg"
import blk_queen from "../assets/blk_queen.svg"
import blk_king from "../assets/blk_king.svg"

import wht_pawn from "../assets/wht_pawn.svg"
import wht_knight from "../assets/wht_knight.svg"
import wht_bishop from "../assets/wht_bishop.svg"
import wht_rook from "../assets/wht_rook.svg"
import wht_queen from "../assets/wht_queen.svg"
import wht_king from "../assets/wht_king.svg"

export function parseFen(fen: string) {
  fen.substring(0, fen.indexOf(" "));
  let res = [];

  for ( let i = 0; i < fen.length; i++ ) {
    let currChar = fen.charAt(i);

    // Check if its 1-8
    // Dont care about other numbers since in fen notation we can see only 1 through 8
    // https://stackoverflow.com/a/52652515
    if (/^[1-8]$/.test(currChar)) {
      for (let i = 0; i < parseInt(currChar); i++ ) {
        res.push(null);
      }
    } else {
      switch (currChar) {
        case 'p':
          res.push(blk_pawn);
          break;
        case 'n':
          res.push(blk_knight);
          break;
        case 'b':
          res.push(blk_bishop);
          break;
        case 'r':
          res.push(blk_rook);
          break;
        case 'q':
          res.push(blk_queen);
          break;
        case 'k':
          res.push(blk_king);
          break;

        case 'P':
          res.push(wht_pawn);
          break;
        case 'N':
          res.push(wht_knight);
          break;
        case 'B':
          res.push(wht_bishop);
          break;
        case 'R':
          res.push(wht_rook);
          break;
        case 'Q':
          res.push(wht_queen);
          break;
        case 'K':
          res.push(wht_king);
          break;
      }
    }
  }

  return res;
}