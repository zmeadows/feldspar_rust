use bitboard::*;
use board::*;
use core::*;
use eval::*;
use moves::*;
use moves::*;
use tables::*;
use game::*;
use movegen::*;
use eval::*;

use std::thread;

struct MiniMaxContext {
    max_depth: usize,
    game: Game,
    move_gen: MoveGen
}

impl MiniMaxContext {

    fn new(new_game: Game, depth: usize) -> MiniMaxContext {
        MiniMaxContext {
            max_depth: depth,
            game: new_game,
            move_gen: MoveGen::new()
        }
    }

    fn maxi(&mut self, depth: usize, move_stack: &MoveStack) -> Score {
        if (depth == self.max_depth) {
            return simple_eval(&self.game);
        }

        let mut max: Score = -9999999999.0;

        self.move_gen.fill_move_buffer(&self.game, move_stack.at_depth(depth));

        for m in move_stack.at_depth(depth).borrow().iter() {
            let game_copy = self.game;

            self.game.make_move(*m);
            let score = self.mini(depth + 1, move_stack);
            self.game = game_copy;

            if score > max {
                max = score;
            }
        }

        return max;
    }

    fn mini(&mut self, depth: usize, move_stack: &MoveStack) -> Score {
        if (depth == self.max_depth) {
            return -1.0 * simple_eval(&self.game);
        }

        let mut min: Score = 9999999999.0;

        self.move_gen.fill_move_buffer(&self.game, move_stack.at_depth(depth));

        for m in move_stack.at_depth(depth).borrow().iter() {
            let game_copy = self.game;

            self.game.make_move(*m);
            let score = self.maxi(depth + 1, move_stack);
            self.game = game_copy;

            if score < min {
                min = score;
            }
        }

        return min;
    }

    // fn go(&mut self, states: Vec<(Move,Game)>) {
    // }

}

// pub fn minimax(game: Game, depth: usize) -> Move {
//     let num_cpus = 5;
//     let chunks = game.next_states_chunked(num_cpus);
// 
//     for move_subset in chunks.iter() {
//     }
// 
//     let mut context = MiniMaxContext::new(game, depth);
// 
//     match game.to_move {
//         Color::White => return 
//         Color::Black =>
//     }
// }

