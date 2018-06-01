extern crate colored;
use print::colored::Colorize;

use core::*;
use bitboard::*;
use board::*;
use moves::*;

impl Piece {
    fn to_unicode(&self) -> &'static str {
        match self.color {
            Color::White => match self.ptype {
                PieceType::Pawn   => "♙ ",
                PieceType::Knight => "♘ ",
                PieceType::Bishop => "♗ ",
                PieceType::Rook   => "♖ ",
                PieceType::Queen  => "♕ ",
                PieceType::King   => "♔ ",
            },
            Color::Black => match self.ptype {
                PieceType::Pawn   => "♟ ",
                PieceType::Knight => "♞ ",
                PieceType::Bishop => "♝ ",
                PieceType::Rook   => "♜ ",
                PieceType::Queen  => "♛ ",
                PieceType::King   => "♚ ",
            }
        }
    }
}

impl Bitboard {

    pub fn print(self) -> () {
        let mut squares = vec![0; 64];

        for sq in self {
            squares[63 - sq.idx()] = 1;
        }

        for row in squares.chunks(8) {
            for x in row {
                print!("{}", x.to_string().color("blue").on_color("white"));
            }
            println!();
        }

        println!();
    }

}

impl Board {
    pub fn print(&self) {
        let mut chars = vec!["  "; 64];
        for i in 0 .. 64 {
            match self.piece_at(Square::new(i)) {
                Some(piece) => chars[63 - i as usize] = piece.to_unicode(),
                None => ()
            }
        }

        let mut row_idx = 8;
        let mut bkg_color = "black";
        println!("   a b c d e f g h");
        for row in chars.chunks(8) {
            match bkg_color {
                "blue" => bkg_color = "white",
                _ => bkg_color = "blue"
            }
            print!("{} ", row_idx.to_string());
            for x in row {
                print!("{}", x.to_string().color("black").on_color(bkg_color).bold());
                match bkg_color {
                    "blue" => bkg_color = "white",
                    _ => bkg_color = "blue"
                }
            }
            print!(" {}", row_idx.to_string());
            row_idx -= 1;
            println!();
        }
        println!("   a b c d e f g h");
    }
}

impl Move {
    pub fn print(&self) {
        println!("FROM: {}, TO: {}", self.from().to_algebraic(), self.to().to_algebraic());
    }
}
