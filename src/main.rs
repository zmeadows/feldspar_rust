#![feature(const_fn)]
#![allow(unused_imports)]
#![feature(extern_prelude)]
#![feature(stdsimd)]
#![feature(iterator_step_by)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate prettytable;
extern crate num_cpus;
extern crate rand;
extern crate time;
use time::PreciseTime;


use std::thread;

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
    init_zobrist_hashing();
    // use Color::*;
    // use PieceType::*;


    let g = Game::from_fen_str("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    g.board.print();
    let mut tree = SearchTree::new(g);
    let mut table = TranspositionTable::new(5000000);
    negamax(&mut tree, 9, Score::min(), Score::max() ).1.print();
    // g.board.print();
    // let (best_score, best_move) = alpha_beta(&mut tree, 5);
    // best_move.print();
    // g.board.print();
    // g.board.attacked_flood(Black, false).print();
    // g.board.attacked(Black, false).print();

    // println!("{:?}", recompute_score(&g.board));

    // let g = Game::starting_position();
    // perft(g, 6);

    // let mut threads = Vec::new();

    // for _ in 0 .. 10 {
    //     let g = Game::starting_position();
    //     threads.push(thread::spawn(move || {
    //         perft(g, 6);
    //     }));
    // }

    // for x in threads {
    //     x.join();
    // }

    //play_against_ai();


    // for m in next_moves_standalone(&g).iter() {
    //     let mut game_copy = g.clone();
    //     game_copy.make_move(*m);
    //     m.print();
    //     game_copy.board.print();
    // }

    // let mut f = Feldspar::new();
    // f.run();

}
