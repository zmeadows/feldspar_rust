#![feature(const_fn)]
#![allow(unused_imports)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate prettytable;
extern crate num_cpus;
extern crate rand;
extern crate time;

mod core; use core::*;
mod board; use board::*;
mod print; use print::*;
mod moves; use moves::*;
mod tables; use tables::*;
mod game; use game::*;
mod bitboard; use bitboard::*;
mod movegen; use movegen::*;
mod perft; use perft::*;
mod play; use play::*;
mod eval; use eval::*;
mod alphabeta; use alphabeta::*;
mod zobrist; use zobrist::*;

fn main() {
    use Color::*;
    use PieceType::*;

    // let mut g = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    // g.board.print();
    // alphabeta(&g,7).print();
    play_against_ai();
}

