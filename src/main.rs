#![feature(const_fn)]

#[macro_use]
extern crate bitflags;

mod core; use core::*;
mod board; use board::*;
mod print; // use print::*;
mod moves; // use moves::*;
mod tables; use tables::*;
mod game; // use game::*;

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

}
