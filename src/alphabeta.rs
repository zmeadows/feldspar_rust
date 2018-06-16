use core::*;
use game::*;
use movegen::*;
use moves::*;

use std::thread;

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
    let num_cpus = num_cpus::get() - 2;
    let next_states = MoveGen::next_states_chunked(&game, false, num_cpus);

    eprintln!("static score: {:?}", game.score);

    if next_states.len() == 0 {
        panic!("finished game passed to alphabeta!");
    }

    let mut threads = Vec::new();

    for move_subset in next_states {

        let to_move = game.to_move;

        threads.push(thread::spawn(move || {
            let mut best_score = match to_move {
                Color::White => Score::min(),
                Color::Black => Score::max()
            };

            let mut best_move = move_subset.first().unwrap().0;

            let move_stack = MoveStack::new();

            for (m, g) in move_subset.iter() {
                let mut context = AlphaBetaContext::new(g, depth - 1);
                let score = match to_move {
                    Color::White => context.mini(Score::min(), Score::max(), 1, &move_stack),
                    Color::Black => context.maxi(Score::min(), Score::max(), 1, &move_stack)
                };

                match to_move {
                    Color::White => if score > best_score {
                        best_score = score;
                        best_move = *m;
                    },
                    Color::Black => if score < best_score {
                        best_score = score;
                        best_move = *m;
                    }
                }
            }

            return (best_move, best_score);
        }));
    }

    let mut results = Vec::new();

    for thread in threads {
        match thread.join() {
            Ok(result) => results.push(result),
            Err(_) => println!("Failed to join threads for alphabeta search.")
        }
    }

    match game.to_move {
        Color::White => results.sort_by(|a, b| b.1.val.partial_cmp(&a.1.val).unwrap()),
        Color::Black => results.sort_by(|a, b| a.1.val.partial_cmp(&b.1.val).unwrap())
    }

    return results.first().unwrap().0;
}

