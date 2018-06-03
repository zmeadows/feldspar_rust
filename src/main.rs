#![feature(const_fn)]

#[macro_use] extern crate prettytable;
#[macro_use] extern crate bitflags;
extern crate num_cpus;

mod core; use core::*;
mod board; use board::*;
mod print; // use print::*;
mod moves; // use moves::*;
mod tables; use tables::*;
mod game; use game::*;
mod bitboard; use bitboard::*;
mod movegen; use movegen::*;
mod perft; use perft::*;

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

    let mut g = Game::from_fen("8/7k/8/8/8/1K6/p7/8 w - - 0 1").unwrap();

    // qperft_debug(g.clone());

    if true {
        g.board.print();
        let mut move_gen = MoveGen::new();
        let move_buffer = move_gen.move_list(&g);

        for m in &move_buffer {
            let mut game_copy = g.clone();
            game_copy.make_move(*m);
            m.print();
            game_copy.board.print();
        }
        println!("moves: {}", move_buffer.len());
    }
}

