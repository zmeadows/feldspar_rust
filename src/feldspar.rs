use game::*;
use movegen::*;
use search::*;
use tree::*;
use uci::*;

use std::str::SplitWhitespace;

pub struct Feldspar {
    tree: GameTree
}

impl Feldspar {
    pub fn new() -> Feldspar {
        Feldspar {
            tree: GameTree::new(Game::starting_position())
        }
    }
}

impl UCIEngine for Feldspar {
    fn name(&self) -> &'static str { "feldspar" }
    fn author(&self) -> &'static str { "Zac Meadows" }

    //TODO: print promotion type!
    fn find_best_move(&mut self) {
        self.tree.trim();

        let (best_move, _) = alpha_beta(&mut self.tree, 5);

        println!( "bestmove {}{}"
                , best_move.from().to_algebraic()
                , best_move.to().to_algebraic()
                );
    }

    fn replace_game(&mut self, new_game: Game) {
        self.tree.reset(new_game);
    }
}
