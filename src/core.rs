use std::slice::Iter;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction { N, S, E, W, NE, NW, SE, SW }

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Square(u32);

impl Square {
    pub fn new(idx: u32) -> Square {
        assert!(idx < 64, "Attempted to create Square with invalid index!");
        Square(idx)
    }

    pub fn idx(self) -> usize { return self.0 as usize; }

    pub fn unwrap(self) -> u32 { return self.0; }

    pub fn rank(self) -> u32 { return self.0/8 + 1; }

    pub fn file(self) -> u32 { return 7 - self.0 % 8; }

    pub fn from_rank_file(rank: u32, file: u32) -> Option<Square> {
        let idx = (rank - 1) * 8 + file;
        if idx < 64 {
            return Some(Square::new(idx));
        } else {
            return None;
        }
    }

    pub fn from_algebraic(alg: &'static str) -> Option<Square> {
        let mut it: Chars = alg.chars();

        let file_idx = match it.next() {
            Some('h') => Some(0),
            Some('g') => Some(1),
            Some('f') => Some(2),
            Some('e') => Some(3),
            Some('d') => Some(4),
            Some('c') => Some(5),
            Some('b') => Some(6),
            Some('a') => Some(7),
            Some(_) => None,
            None => None
        };

        let rank_idx: Option<u32> = match it.next() {
            Some(x) => x.to_digit(10),
            None => None
        };


        match file_idx {
            None => return None,
            Some(fid) => {
                match rank_idx {
                    None => return None,
                    Some(rid) => return Square::from_rank_file(rid, fid)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl PieceType {
    pub fn all() -> Iter<'static, PieceType> {
        use self::PieceType::*;
        static PIECETYPES: [PieceType;  6] = [Pawn, Knight, Bishop, Rook, Queen, King];
        PIECETYPES.into_iter()
    }

    pub fn is_slider(self) -> bool {
        use self::PieceType::*;
        match self {
            Pawn   => return false,
            Knight => return false,
            Bishop => return true,
            Rook   => return true,
            Queen  => return true,
            King   => return false
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color { White, Black }

use std::ops::Not;
impl Not for Color {
    type Output = Color;
    fn not(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Piece {
    pub ptype: PieceType,
    pub color: Color,
}

