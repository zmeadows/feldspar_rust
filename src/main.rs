#![feature(const_fn)]

mod core; use core::*;
mod board; use board::*;
mod print; // use print::*;
mod moves; // use moves::*;
mod tables; // use tables::*;
mod game; // use game::*;

fn main() {
    let b = Board::starting_position();
    b.occupied().print();
    b.unoccupied().print();
    b.get_pieces(Color::White, PieceType::Pawn).print();
    b.print();
}
