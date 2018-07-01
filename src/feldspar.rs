use eval::*;
use core::*;
use game::*;
use movegen::*;
use moves::*;
use search::*;
use tree::*;
use uci::*;
use zobrist::*;

use std::time::Instant;
use std::cmp::max;

use std::str::SplitWhitespace;

pub struct Feldspar {
    context: SearchContext
}

impl Feldspar {
    pub fn new() -> Feldspar {
        let mut tmp_tree = SearchTree::new(Game::starting_position());
        let mut tmp_qtree = SearchTree::new(Game::starting_position());
        tmp_qtree.in_quiescence = true;
        let mut tmp_table = TranspositionTable::new(100000000);

        let mut new_context = SearchContext {
            tree: tmp_tree,
            qtree: tmp_qtree,
            table: tmp_table,
            timer: SearchTimer::new(3000),
            ran_out_of_time: false
        };

        Feldspar {
            context: new_context
        }
    }
}

impl UCIEngine for Feldspar {
    fn name(&self) -> &'static str { "feldspar" }
    fn author(&self) -> &'static str { "Zac Meadows" }

    //TODO: print promotion type!
    fn find_best_move(&mut self, wtime: u32, btime: u32, winc: u32, binc: u32) -> () {

        let mut my_time = 0;
        let mut opp_time = 0;
        let mut my_inc = 0;
        let mut opp_inc = 0;

        if self.context.tree.focus().to_move == Color::White {
            my_time = wtime;
            opp_time = btime;
            my_inc = winc;
            opp_inc = binc;
        } else {
            my_time = btime;
            opp_time = wtime;
            my_inc = binc;
            opp_inc = winc;
        }

        if my_time > opp_time {
            self.context.timer = SearchTimer::new( max(my_time - opp_time, my_time/50) );
        } else {
            if my_time > 10000 {
                self.context.timer = SearchTimer::new( max(my_time/40, 1500) );
            } else {
                self.context.timer = SearchTimer::new( max(my_time/40, 500) );
            }
        }

        self.context.ran_out_of_time = false;

        let mut depth_reached = 0;
        let mut best_move = Move::null();
        let mut best_score = Score::min();

        for i in 1 .. 999 {
            negamax( &mut self.context, i, Score::min(), Score::max() );
            if !self.context.ran_out_of_time {
                depth_reached = i;
                let pv = self.context.table.get_pv(*self.context.tree.focus(), depth_reached as usize);
                if pv.len() > 0 {
                    best_move = pv[0].best_move();
                    best_score = pv[0].score();

                    let mut pv_str = String::new();

                    for entry in pv.iter() {
                        if pv_str.len() > 0 {
                            pv_str.push_str(" ");
                        }
                        pv_str.push_str(&entry.best_move().to_uci_str());
                    }

                    println!("info depth {} score cp {} pv {}", depth_reached, best_score.unwrap(), pv_str);
                    eprintln!("best_move from negamax: {}{}", best_move.from().to_algebraic(), best_move.to().to_algebraic());
                }
            } else {
                break;
            }
        }

        // match self.context.tree.focus().to_move {
        //     Color::White => eprintln!("score: {:?}", (best_score.unwrap() as f32)/100.0),
        //     Color::Black => eprintln!("score: {:?}", (best_score.flipped().unwrap() as f32)/100.0)
        // }

        println!( "bestmove {}{}"
                , best_move.from().to_algebraic()
                , best_move.to().to_algebraic()
                );

        self.context.ran_out_of_time = false;

        //TODO: ponder while opponent thinks
    }

    fn replace_game(&mut self, new_game: Game, history: Vec<Hash>) {
        self.context.tree.reset_root(new_game, history);
    }
}
