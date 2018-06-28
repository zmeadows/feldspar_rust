use eval::*;
use game::*;
use movegen::*;
use moves::*;
use search::*;
use tree::*;
use uci::*;
use zobrist::*;

use std::str::SplitWhitespace;

pub struct Feldspar {
    tree: SearchTree,
    table: TranspositionTable
}

impl Feldspar {
    pub fn new() -> Feldspar {
        Feldspar {
            tree: SearchTree::new(Game::starting_position()),
            table: TranspositionTable::new(50000000)
        }
    }
}

impl UCIEngine for Feldspar {
    fn name(&self) -> &'static str { "feldspar" }
    fn author(&self) -> &'static str { "Zac Meadows" }

    //TODO: print promotion type!
    fn find_best_move(&mut self) {
        let (best_score, best_move) = negamax(&mut self.tree, &mut self.table, 6, Score::min(), Score::max());

        println!( "bestmove {}{}"
                , best_move.from().to_algebraic()
                , best_move.to().to_algebraic()
                );

        eprintln!("static score: {:?}", Score::recompute(&self.tree.focus(), 0));
        eprintln!("search score: {:?}", best_score);
    }

    fn replace_game(&mut self, new_game: Game, moves: Vec<Move>) {
        self.tree.reset_root(new_game, moves);
    }
}
