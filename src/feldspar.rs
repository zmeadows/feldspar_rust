use game::*;
use movegen::*;
use search::*;
use tree::*;
use uci::*;
use eval::*;

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

        let (best_move, best_score) = alpha_beta(&mut self.tree, 6);

        println!( "bestmove {}{}"
                , best_move.from().to_algebraic()
                , best_move.to().to_algebraic()
                );

        eprintln!("static score: {:?}", Score::recompute(&self.tree.game));
        eprintln!("best ab score: {:?}", best_score);
    }

    fn replace_game(&mut self, new_game: Game) {
        self.tree.reset(new_game);
    }
}
