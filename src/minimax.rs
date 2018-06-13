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
            return simple_eval(&self.game);
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
}

pub fn minimax(game: &Game, depth: usize) -> Move {
    let next_states = MoveGen::next_states(&game);

    if next_states.len() == 0 {
        panic!("invalid game passed to minimax!");
    }

    let mut best_move = next_states.first().unwrap().0;
    let mut best_score = match game.to_move {
        Color::White => -999999999999999.0,
        Color::Black => 999999999999999.0
    };

    let move_stack = MoveStack::new();

    for (move_candidate, game_candidate) in MoveGen::next_states(&game) {
        let mut context = MiniMaxContext::new(game_candidate, depth - 1);
        match game.to_move {
            Color::White => {
                let new_score = context.mini(1, &move_stack);
                if (new_score > best_score) {
                    best_move = move_candidate;
                    best_score = new_score;
                }
            },
            Color::Black => {
                let new_score = context.maxi(1, &move_stack);
                if (new_score < best_score) {
                    best_move = move_candidate;
                    best_score = new_score;
                }
            }
        }
    }

    println!("best score: {}", best_score);

    return best_move;
}

