use core::*;
use game::*;
use movegen::*;
use moves::*;

// use std::thread;

struct AlphaBetaContext {
    max_depth: usize,
    game: Game,
    move_gen: MoveGen
}

impl AlphaBetaContext {

    fn new(new_game: &Game, depth: usize) -> AlphaBetaContext {
        AlphaBetaContext {
            max_depth: depth,
            game: new_game.clone(),
            move_gen: MoveGen::new()
        }
    }

    fn maxi(&mut self, mut alpha: Score, beta: Score, depth: usize, move_stack: &MoveStack) -> Score {
        if depth == self.max_depth {
            return self.game.score;
        }

        self.move_gen.fill_move_buffer(&self.game, move_stack.at_depth(depth));

        for m in move_stack.at_depth(depth).borrow().iter() {
            let game_copy = self.game;

            self.game.make_move(*m);
            let score = self.mini(alpha, beta, depth + 1, move_stack);
            self.game = game_copy;

            if score >= beta {
                return beta;   // fail hard beta-cutoff
            }

            if score > alpha {
                alpha = score; // alpha acts like max in MiniMax
            }

        }

        return alpha;
    }

    fn mini(&mut self, alpha: Score, mut beta: Score, depth: usize, move_stack: &MoveStack) -> Score {
        if depth == self.max_depth {
            return self.game.score;
        }

        self.move_gen.fill_move_buffer(&self.game, move_stack.at_depth(depth));

        for m in move_stack.at_depth(depth).borrow().iter() {
            let game_copy = self.game;

            self.game.make_move(*m);
            let score = self.maxi(alpha, beta, depth + 1, move_stack);
            self.game = game_copy;

            if score <= alpha {
                return alpha; // fail hard alpha-cutoff
            }
            if score < beta {
                beta = score; // beta acts like min in MiniMax
            }

        }

        return beta;
    }
}

pub fn alphabeta(game: &Game, depth: usize) -> Move {
    let next_states = MoveGen::next_states(&game);

    if next_states.len() == 0 {
        panic!("finished game passed to alphabeta!");
    }

    let mut move_scores: Vec<(Move, Score)> = MoveGen::next_states(&game).iter().map(
        |(move_candidate, game_candidate)| -> (Move, Score) {
            let mut context = AlphaBetaContext::new(game_candidate, depth - 1);
            let move_stack = MoveStack::new();

            let score = match game.to_move {
                Color::White => context.mini(Score::min(), Score::max(), 1, &move_stack),
                Color::Black => context.maxi(Score::min(), Score::max(), 1, &move_stack)
            };

            return (*move_candidate, score);
        }).collect();

    match game.to_move {
        Color::White => move_scores.sort_by(|a, b| b.1.val.partial_cmp(&a.1.val).unwrap()),
        Color::Black => move_scores.sort_by(|a, b| a.1.val.partial_cmp(&b.1.val).unwrap())
    }

    println!("ab score: {:?}", move_scores.first().unwrap().1);
    return move_scores.first().unwrap().0;
}

