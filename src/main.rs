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
extern crate chrono;

use std::fs::File;
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
    use Color::*;
    use PieceType::*;

    // let g = Game::starting_position();
    //let g = Game::from_fen_str("8/1p5p/1p3k2/2n3p1/2PN2P1/P3r2P/1P4K1/5R2 b - - 5 41").unwrap();
    //let g = Game::random_game();
    //let g = Game::from_fen_str("5bk1/2R1pp2/6p1/3PP2p/P3B3/5P1P/1r2K3/8 w - - 7 43").unwrap();
    //println!("{}", g.to_fen());
    // g.board.print();

    // let mut tmp_tree = SearchTree::new(g);
    // let mut tmp_qtree = SearchTree::new(g);
    // tmp_qtree.in_quiescence = true;
    // let mut tmp_table = TranspositionTable::new(20000000);

    // let mut context = SearchContext {
    //     tree: tmp_tree,
    //     qtree: tmp_qtree,
    //     table: tmp_table,
    //     timer: SearchTimer::new(300)
    // };

    // for i in 1..15 {
    //      let (s,m) = negamax(&mut context, i, Score::min(), Score::max() );
    //      m.print();
    //      //let best_move = context.table.get_pv(*context.tree.focus())[0];
    //      if context.timer.finished() {
    //          break;
    //      }
    // }

    // g.board.print();
    // let (best_score, best_move) = alpha_beta(&mut tree, 5);
    // best_move.print();
    // g.board.print();
    // g.board.attacked_flood(Black, false).print();
    // g.board.attacked(Black, false).print();

    // println!("{:?}", recompute_score(&g.board));

    // let g = Game::starting_position();
    // perft(g, 5);



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

    let mut f = Feldspar::new();
    f.run();

}
