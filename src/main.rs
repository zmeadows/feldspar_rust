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

#[macro_use] extern crate clap;
use clap::App;

use std::fs::File;
use std::thread;
use std::process;

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

const FELDSPAR_VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    init_zobrist_hashing();
    use Color::*;
    use PieceType::*;

    if matches.is_present("ponder") {
        let ponder_FEN = matches.value_of("ponder").unwrap();
        match Game::from_fen_str(ponder_FEN) {
            None => {
                eprintln!("Invalid FEN string passed: {}", ponder_FEN);
                process::exit(1);
            }

            Some(game) => {
                game.board.print();
                println!("{}", game.to_fen());
                let mut tmp_tree = SearchTree::new(game);
                let mut tmp_qtree = SearchTree::new(game);
                tmp_qtree.in_quiescence = true;
                let mut tmp_table = TranspositionTable::new(20000000);

                let mut context = SearchContext {
                    tree: tmp_tree,
                    qtree: tmp_qtree,
                    table: tmp_table,
                    timer: SearchTimer::new(u32::max_value()),
                    ran_out_of_time: false
                };

                for i in 1 .. {
                    let (s,m) = negamax(&mut context, i, Score::min(), Score::max());
                    m.print();
                }
            }
        }
    } else if matches.is_present("perft") {
    } else if matches.is_present("uci") {
        Feldspar::new().run();
    }


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


}
