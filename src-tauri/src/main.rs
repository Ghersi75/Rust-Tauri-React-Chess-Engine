// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// TODO: Remove and check that everything is being used properly
#![allow(unused)]

mod board;
use board::board::BoardState;


fn main() {
    match BoardState::init_default() {
        Ok(board) => board.print_board(),
        Err(e) => println!("{e}")
    }
    chess_lib::run()
}
