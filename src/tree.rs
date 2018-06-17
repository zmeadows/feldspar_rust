use core::*;
use bitboard::*;
use moves::*;
use move_list::*;
use movegen::*;
use board::*;
use tables::*;
use game::*;
use pins::*;

const MAX_GAME_TREE_DEPTH: usize = 256;

pub struct GameTree {
    pub game: Game,
    pub depth: usize,
    move_stack: Vec<MoveBuffer>
}

impl GameTree {
    pub fn focus(&self) -> &Game {
        &self.game
    }

    pub fn new(new_game: Game) -> GameTree {
        let mut new_move_stack = Vec::new();
        for _ in 0 .. MAX_GAME_TREE_DEPTH {
            new_move_stack.push(alloc_move_buffer());
        }

        GameTree {
            game: new_game,
            depth: 0,
            move_stack: new_move_stack,
        }
    }

    pub fn next_moves(&self) -> MoveBuffer {
        {
            let mut buf = self.move_stack[self.depth].borrow_mut();

            if buf.stage != MoveGenStage::Finished {
                buf.generate_moves(&self.game, false);
            }

            assert!(buf.stage == MoveGenStage::Finished);

            buf.list.sort();
        }

        self.move_stack[self.depth].clone()
    }

    pub fn make_move(&mut self, m: Move) {
        self.game.make_move(m);
        self.depth += 1;

        let mut buf = self.move_stack[self.depth].borrow_mut();
        buf.clear();
        let can_move = buf.generate_moves(&self.game, true);

        // no moves available, game is over
        if !can_move {
            assert!(buf.stage == MoveGenStage::Finished);
            let check_multiplicity = self.game.king_attackers.population();
            if check_multiplicity > 0 {
                // check mate
                match self.game.to_move {
                    Color::White => self.game.score = Score::min(),
                    Color::Black => self.game.score = Score::max()
                }
            } else {
                // stale mate
                self.game.score = Score::new(0);
            }
        }
    }

    pub fn unmake_move(&mut self, new_game: Game) {
        assert!(self.depth > 0);
        self.move_stack[self.depth].borrow_mut().clear();
        self.depth -= 1;
        self.game = new_game;
    }

    pub fn reset(&mut self, new_game: Game) {
        self.game = new_game;
        self.depth = 0;
        for buf in self.move_stack.iter() {
            buf.borrow_mut().clear();
        }
    }

    pub fn trim(&mut self) {
        self.depth = 0;
        for buf in self.move_stack.iter() {
            buf.borrow_mut().clear();
        }
    }
}

