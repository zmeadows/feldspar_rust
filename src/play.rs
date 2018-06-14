use bitboard::*;
use board::*;
use core::*;
use game::*;
use movegen::*;
use moves::*;
use tables::*;
use alphabeta::*;


pub fn play_against_ai() {
    let mut game = Game::from_fen("r2qnrk1/3nbppp/3pb3/p3p1P1/1p2PP2/1N2B3/PPPQN2P/2KR1B1R w - - 0 15").unwrap();

    loop {
        game.board.print();
        use std::io::{stdin,stdout,Write};
        let mut s=String::new();
        print!("Enter your move: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        match move_from_algebraic(game, s) {
            Some(m) => {
                game.make_move(m);
                let ai_move = alphabeta(&game,8);
                game.make_move(ai_move);
            },
            None => println!("Invalid move!")
        }
    }

}

// use rand::{thread_rng, ThreadRng, Rng};

// pub struct MCTS {
//     move_gen: MoveGen,
//     rng: ThreadRng,
//     move_buffer: MoveBuffer
// }
// 
// impl MCTS {
//     pub fn new() -> MCTS {
//         MCTS {
//             move_gen: MoveGen::new(),
//             rng: thread_rng(),
//             move_buffer: alloc_move_buffer()
//         }
//     }
// 
//     pub fn play_random_game(&mut self, mut game: Game) -> GameResult {
//         while true {
//             self.move_gen.fill_move_buffer(&game, &self.move_buffer);
//             match game.outcome(self.move_buffer.borrow().len()) {
//                 Some(result) => {
//                     println!("{}", game.moves_played);
//                     return result;
//                 }
//                 None => {}
//             }
// 
//             let num_moves = self.move_buffer.borrow().len();
// 
//             let n = if (num_moves == 1) { 0 } else { self.rng.gen_range(0, num_moves - 1) };
// 
//             game.make_move(self.move_buffer.borrow().at(n));
//         }
// 
//         return GameResult::Draw;
//     }
// }

