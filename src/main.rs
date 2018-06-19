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

    let g = Game::from_fen_str("r3k2r/6pp/pp2n3/2qbpn1P/8/P4P1B/2PNN1K1/R1Q4R b kq - 5 27").unwrap();
    g.board.print();

    let mut tree = SearchTree::new(g);
    let (best_score, best_move) = alpha_beta(&mut tree, 6);
    best_move.print();

    // println!("{:?}", recompute_score(&g.board));

    //let g = Game::from_fen_str("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    // let g = Game::starting_position();
    // perft(g, 5);
    //g.board.print();
    //alphabeta(&g,7).print();
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
