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
            self.context.timer = SearchTimer::new(my_time - opp_time);
        } else {
            self.context.timer = SearchTimer::new( max(my_time/40, 500) );
        }

        let mut depth_reached = 0;
        let mut best_move = Move::null();
        let mut best_score = Score::min();
        for i in 1 .. 100 {
            let (s,m) = negamax( &mut self.context, i, Score::min(), Score::max() );
            if !self.context.ran_out_of_time {
                best_move = m;
                best_score = s;
                depth_reached = i;
            } else {
                break;
            }
        }

        let best_move = self.context.table.get_pv(*self.context.tree.focus(), depth_reached as usize)[0];

        match self.context.tree.focus().to_move {
            Color::White => eprintln!("score: {:?}", (best_score.unwrap() as f32)/100.0),
            Color::Black => eprintln!("score: {:?}", (best_score.flipped().unwrap() as f32)/100.0)
        }

        println!( "bestmove {}{}"
                , best_move.from().to_algebraic()
                , best_move.to().to_algebraic()
                );

        self.context.ran_out_of_time = false;

        //TODO: ponder while opponent thinks
    }

    fn replace_game(&mut self, new_game: Game, history: Vec<Move>) {
        self.context.tree.reset_root(new_game);
    }
}
