#![feature(const_fn)]
#![allow(unused_imports)]
#![feature(extern_prelude)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate prettytable;
extern crate num_cpus;
extern crate rand;
extern crate time;

mod search; use search::*;
mod bitboard; use bitboard::*;
mod board; use board::*;
mod core; use core::*;
mod eval; use eval::*;
mod feldspar; use feldspar::*;
mod game; use game::*;
mod movegen; use movegen::*;
mod moves; use moves::*;
mod move_list; use move_list::*;
mod perft; use perft::*;
mod pins; use pins::*;
mod play; use play::*;
mod print; use print::*;
mod tables; use tables::*;
mod uci; use uci::*;
mod zobrist; use zobrist::*;
mod tree; use tree::*;

fn main() {
    use Color::*;
    use PieceType::*;

    // let g = Game::from_fen_str("4r1Rk/1R5p/p7/8/P1P2q2/6QP/6PK/8 b - - 0 39").unwrap();

    // g.board.print();
    // println!("{:?}", recompute_score(&g.board));

    //let g = Game::from_fen_str("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    // let g = Game::starting_position();
    // perft(g, 6);
    //g.board.print();
    //alphabeta(&g,7).print();
    // play_against_ai();


    //     for m in next_moves_standalone(&g).iter() {
    //         let mut game_copy = g.clone();
    //         game_copy.make_move(*m);
    //         m.print();
    //         game_copy.board.print();
    //     }

    let mut f = Feldspar::new();
    f.run();
}

