#![feature(const_fn)]

#[macro_use]
extern crate bitflags;

mod core; use core::*;
mod board; use board::*;
mod print; // use print::*;
mod moves; // use moves::*;
mod tables; use tables::*;
mod game; use game::*;
mod bitboard; use bitboard::*;
mod movegen; use movegen::*;

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

    let g = Game::from_fen("bk2r3/7q/8/3QRB2/r2QKN1r/3PQR2/8/1b2r2q w - - 1 1").unwrap();
    //let g = Game::starting_position();
    g.board.get_pieces(Color::Black, PieceType::Knight).print();
    g.board.get_pieces(Color::White, PieceType::Knight).print();
    g.board.get_pieces(Color::White, PieceType::Pawn).print();
    g.board.get_pieces(Color::Black, PieceType::Pawn).print();
    g.board.occupied().print();
    g.board.occupied_by(Color::White).print();
    g.board.occupied_by(Color::Black).print();
    g.board.unoccupied().print();
    g.board.print();
    let mut x = Vec::new();
    g.generate_moves(&mut x);
    println!("{}", x.len());

    for m in x { m.print(); }



}
