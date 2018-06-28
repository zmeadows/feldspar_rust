#![feature(const_fn)]
#![feature(reverse_bits)]
#![allow(unused_imports)]
#![feature(extern_prelude)]
#![feature(stdsimd)]
#![feature(iterator_step_by)]
#![feature(plugin, custom_attribute)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate prettytable;
extern crate num_cpus;
extern crate rand;
extern crate time;

use std::fs::File;
use std::thread;
use time::PreciseTime;

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
    use Color::*;
    use PieceType::*;

    //let g = Game::random_game();
    let g = Game::from_fen_str("bn2kbnr/2p1pppp/rp6/p2p2N1/QPP5/N2P1PP1/P3P1P1/R1B1KB1R b KQk - 2 9").unwrap();
    println!("{}", g.to_fen());
    g.board.print();

    let mut tree = SearchTree::new(g);
    let mut table = TranspositionTable::new(200000000);
    for i in 1..10 {
        negamax(&mut tree, &mut table, i, Score::min(), Score::max() ).1.print();
    }

    // g.board.print();
    // let (best_score, best_move) = alpha_beta(&mut tree, 5);
    // best_move.print();
    // g.board.print();
    // g.board.attacked_flood(Black, false).print();
    // g.board.attacked(Black, false).print();

    // println!("{:?}", recompute_score(&g.board));

    // let g = Game::starting_position();
    // perft(g, 6);



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
