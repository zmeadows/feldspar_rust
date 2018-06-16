use alphabeta::*;
use game::*;
use uci::*;
use movegen::*;

use std::str::SplitWhitespace;

pub struct Feldspar {
    game: Game
}

impl Feldspar {
    pub fn new() -> Feldspar {
        Feldspar {
            game: Game::starting_position()
        }
    }
}

impl UCIEngine for Feldspar {
    fn name(&self) -> &'static str { "feldspar" }
    fn author(&self) -> &'static str { "Zac Meadows" }
    //TODO: print promotion type!
    fn find_best_move(&mut self) {
        let best_move = alphabeta(&self.game, 7);
        println!("bestmove {}{}", best_move.from().to_algebraic(),
                                  best_move.to().to_algebraic());
    }

    fn root_node(&mut self) -> &mut Game {
        return &mut self.game;
    }

}
