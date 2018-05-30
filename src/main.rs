#![feature(const_fn)]

#[macro_use]
extern crate bitflags;

mod core; use core::*;
mod board; use board::*;
mod print; // use print::*;
mod moves; // use moves::*;
mod tables; use tables::*;
mod game; use game::*;

fn main() {
    let b = Board::starting_position();
    // b.occupied().print();
    // b.unoccupied().print();
    // b.get_pieces(Color::White, PieceType::Pawn).print();
    get_positive_ray(Square::new(17), Direction::N, b.occupied()).print();
    get_positive_ray(Square::new(17), Direction::NW, b.occupied()).print();
    get_positive_ray(Square::new(17), Direction::NE, b.occupied()).print();

    get_negative_ray(Square::new(27), Direction::S, b.occupied()).print();
    get_negative_ray(Square::new(27), Direction::SW, b.occupied()).print();
    get_negative_ray(Square::new(27), Direction::SE, b.occupied()).print();

    WHITE_KINGSIDE_CASTLE_BITS.print();
    WHITE_QUEENSIDE_CASTLE_BITS.print();
    BLACK_KINGSIDE_CASTLE_BITS.print();
    BLACK_QUEENSIDE_CASTLE_BITS.print();

    let g = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    if (g.is_some()) {
        g.unwrap().board.get_pieces(Color::Black, PieceType::Knight).print();
        g.unwrap().board.get_pieces(Color::White, PieceType::Knight).print();
        g.unwrap().board.get_pieces(Color::White, PieceType::Pawn).print();
        g.unwrap().board.print();
    }

}
