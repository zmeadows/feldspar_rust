#![feature(const_fn)]
#![allow(unused_imports)]
#![feature(extern_prelude)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate prettytable;
extern crate num_cpus;
extern crate rand;
extern crate time;

mod alphabeta; use alphabeta::*;
mod bitboard; use bitboard::*;
mod board; use board::*;
mod core; use core::*;
mod eval; use eval::*;
mod feldspar; use feldspar::*;
mod game; use game::*;
mod movegen; use movegen::*;
mod moves; use moves::*;
mod perft; use perft::*;
mod pins; use pins::*;
mod play; use play::*;
mod print; use print::*;
mod tables; use tables::*;
mod uci; use uci::*;
mod zobrist; use zobrist::*;

fn main() {
    use Color::*;
    use PieceType::*;

    //let g = Game::from_fen_str("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    let g = Game::starting_position();
    perft(g, 6);
    //g.board.print();
    //alphabeta(&g,7).print();
    //play_against_ai();

    
    // if true {
    //     g.board.print();
    //     let mut move_gen = MoveGen::new();
    //     let move_buffer = alloc_move_buffer();
    //     move_gen.fill_move_buffer(&g, &move_buffer);

    //     for m in move_buffer.borrow().iter() {
    //         let mut game_copy = g.clone();
    //         game_copy.make_move(*m);
    //         m.print();
    //         game_copy.board.print();
    //     }
    //     println!("moves: {}", move_buffer.borrow().len());
    // }

    // let mut f = Feldspar::new();
    // f.run();
}

