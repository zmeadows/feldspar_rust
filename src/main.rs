#![feature(const_fn)]

#[macro_use] extern crate prettytable;
#[macro_use] extern crate bitflags;
extern crate num_cpus;
extern crate time;
extern crate rand;

mod core; use core::*;
mod board; use board::*;
mod print; // use print::*;
mod moves; use moves::*;
mod tables; use tables::*;
mod game; use game::*;
mod bitboard; use bitboard::*;
mod movegen; use movegen::*;
mod perft; use perft::*;
mod play; use play::*;
mod eval; use eval::*;
mod minimax; use minimax::*;

fn main() {
    let b = Board::starting_position();
    // b.occupied().print();
    // b.unoccupied().print();
    // b.get_pieces(Color::White, PieceType::Pawn).print();
    // get_positive_ray(Square::new(17), Direction::N, b.occupied()).print();
    // get_positive_ray(Square::new(17), Direction::NW, b.occupied()).print();
    // get_positive_ray(Square::new(17), Direction::NE, b.occupied()).print();

    // get_negative_ray(Square::new(27), Direction::S, b.occupied()).print();
    // get_negative_ray(Square::new(27), Direction::SW, b.occupied()).print();
    // get_negative_ray(Square::new(27), Direction::SE, b.occupied()).print();

    // WHITE_KINGSIDE_CASTLE_BITS.print();
    // WHITE_QUEENSIDE_CASTLE_BITS.print();
    // BLACK_KINGSIDE_CASTLE_BITS.print();
    // BLACK_QUEENSIDE_CASTLE_BITS.print();

    use Color::*;
    use PieceType::*;

    //let mut g = Game::from_fen("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1").unwrap();
    // let mut g = Game::from_fen("rnbqk1nr/pppp1ppp/8/4p3/1b1P4/8/PPPQPPPP/RNB1KBNR w KQkq - 0 1").unwrap();
    // println!("{}", g.to_fen());

    let mut g = Game::from_fen("rnb1kbnr/pp2pppp/1q1p4/2p1N3/3PP3/8/PPP2PPP/RNBQKB1R b KQkq - 2 4").unwrap();
    g.board.print();

    // perft(g.clone(),6);

    //let mut mcts = MCTS::new();

    // for i in 0 .. 10000 {
    //     mcts.play_random_game(g);
    // }

    minimax(&g,5).print();

    if false {
        g.board.print();
        let mut move_gen = MoveGen::new();
        let move_buffer = alloc_move_buffer();
        move_gen.fill_move_buffer(&g, &move_buffer);

        for m in move_buffer.borrow().iter() {
            let mut game_copy = g.clone();
            game_copy.make_move(*m);
            m.print();
            game_copy.board.print();
        }
        println!("moves: {}", move_buffer.borrow().len());
    }

}

