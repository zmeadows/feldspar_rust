use bitboard::*;
use board::*;
use core::*;
use game::*;
use movegen::*;
use moves::*;
use tables::*;

use rand::{thread_rng, ThreadRng, Rng};

pub struct MCTS {
    move_gen: MoveGen,
    rng: ThreadRng,
    move_buffer: MoveBuffer
}

impl MCTS {
    pub fn new() -> MCTS {
        MCTS {
            move_gen: MoveGen::new(),
            rng: thread_rng(),
            move_buffer: alloc_move_buffer()
        }
    }

    pub fn play_random_game(&mut self, mut game: Game) -> GameResult {
        while true {
            self.move_gen.fill_move_buffer(&game, &self.move_buffer);
            match game.outcome(self.move_buffer.borrow().len()) {
                Some(result) => {
                    println!("{}", game.moves_played);
                    return result;
                }
                None => {}
            }

            let num_moves = self.move_buffer.borrow().len();

            let n = if (num_moves == 1) { 0 } else { self.rng.gen_range(0, num_moves - 1) };

            game.make_move(self.move_buffer.borrow().at(n));
        }

        return GameResult::Draw;
    }
}

