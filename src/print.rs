extern crate colored;
use print::colored::Colorize;

use core::*;
use board::*;

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

        let mut bkg_color = "black";
        for row in chars.chunks(8) {
            match bkg_color {
                "black" => bkg_color = "red",
                _ => bkg_color = "black"
            }
            for x in row {
                print!("{}", x.to_string().color("white").on_color(bkg_color).bold());
                match bkg_color {
                    "black" => bkg_color = "red",
                    _ => bkg_color = "black"
                }
            }
            println!();
        }
    }
}
