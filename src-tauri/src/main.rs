// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// TODO: Remove and check that everything is being used properly
#![allow(unused)]

mod board;

fn main() {
    chess_lib::run()
}
