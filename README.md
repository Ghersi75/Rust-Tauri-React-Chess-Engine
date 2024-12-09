# Rust Chess Engine

This is my attempt at making a rust chess engine. The backend/engine logic will be fully implemented in Rust. Since the frontend isn't the important part here, I used React and Tailwind as I'm familiar with them and I could get this part done fast. 

## Project Layout
- `/src` The frontend. Implemented in React + Tailwind
- `/src-tauri` The backend. Implemented in Rust
  - `board/` The main board state related files
    - `bitboard.rs` Bitboard representation and helper functions
    - `board.rs` Board state representation and helper functions