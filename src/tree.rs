use core::*;
use bitboard::*;
use moves::*;
use move_list::*;
use movegen::*;
use board::*;
use tables::*;
use game::*;
use pins::*;
use eval::*;
use zobrist::*;

const MAX_GAME_TREE_DEPTH: usize = 256;
const MAX_CHESS_GAME_LENGTH: usize = 550;

pub struct SearchTree {
    game: Game,
    search_depth: usize,
    pub root_history: Vec<Hash>,
    pub current_line: Vec<Move>,
    best_lines: Vec<(Score,MoveList)>,
    move_stack: Vec<MoveBuffer>,
    pub in_quiescence: bool
}

impl SearchTree {
    pub fn focus(&self) -> &Game {
        &self.game
    }

    pub fn last_move(&self) -> Move {
        *self.current_line.last().unwrap()
    }

    pub fn search_depth(&self) -> usize {
        self.search_depth
    }

    pub fn new(new_game: Game) -> SearchTree {
        let mut new_move_stack = Vec::new();
        new_move_stack.reserve(MAX_GAME_TREE_DEPTH);
        for _ in 0 .. MAX_GAME_TREE_DEPTH {
            new_move_stack.push(alloc_move_buffer());
        }

        let mut new_current_line = Vec::new();
        new_current_line.reserve(MAX_GAME_TREE_DEPTH);

        SearchTree {
            game: new_game,
            search_depth: 0,
            current_line: new_current_line,
            best_lines: Vec::new(),
            root_history: Vec::new(),
            move_stack: new_move_stack,
            in_quiescence: false
        }
    }

    pub fn next_moves(&self, best_move_candidate: Option<Move>) -> MoveBuffer {
        {
            let buf = self.move_stack[self.search_depth].clone();
            if self.in_quiescence {
                //TODO: handle checks in quiescence
                generate_moves(&self.game, buf.clone(), true);
            } else {
                generate_moves(&self.game, buf.clone(), false);
            }
            buf.borrow_mut().sort(best_move_candidate);
        }

        self.move_stack[self.search_depth].clone()
    }

    pub fn make_null_move(&mut self) {
        self.game.make_null_move();
        self.search_depth += 1;
        self.move_stack[self.search_depth].borrow_mut().clear();
    }

    pub fn make_move(&mut self, m: Move) {
        self.game.make_move(m);
        self.current_line.push(m);
        self.search_depth += 1;
        self.move_stack[self.search_depth].borrow_mut().clear();

        if !self.in_quiescence {
            let mut repetition_count = 1;
            for h in self.root_history.iter() {
                if *h == self.game.hash {
                    repetition_count += 1;
                }
            }

            if repetition_count >= 3 {
                self.game.outcome = Some(GameResult::Draw);
            }

            self.root_history.push(self.game.hash);
        }

        //TODO: check for three-fold repetition here.
    }

    pub fn unmake_null_move(&mut self, previous_game: Game) {
        debug_assert!(self.search_depth > 0);
        self.move_stack[self.search_depth].borrow_mut().clear();
        self.search_depth -= 1;
        self.game = previous_game;
    }

    // currently we unmake move by copy
    // OPTIMIZE: is this copying twice??? nail down rust copy/move semantics
    pub fn unmake_move(&mut self, previous_game: Game) {
        debug_assert!(self.search_depth > 0);
        self.move_stack[self.search_depth].borrow_mut().clear();
        self.search_depth -= 1;
        self.game = previous_game;
        self.current_line.pop();
        self.root_history.pop();
    }

    pub fn reset_root(&mut self, new_game: Game, history: Vec<Hash>) {
        self.game = new_game;
        self.search_depth = 0;
        self.current_line.clear();
        self.root_history = history.clone();

        for (_, buf) in self.move_stack.iter().enumerate() {
            buf.borrow_mut().clear();
        }
    }
}

