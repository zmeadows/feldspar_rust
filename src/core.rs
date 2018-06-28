use std::ops::Not;
use std::slice::Iter;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction { N, S, E, W, NE, NW, SE, SW }

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Square(u32);

impl Square {
    pub fn new(idx: u32) -> Square {
        debug_assert!(idx < 64, "Attempted to create Square with invalid index! {}", idx);
        Square(idx)
    }

    pub fn idx(self) -> usize { return self.0 as usize; }

    pub fn unwrap(self) -> u32 { return self.0; }

    pub fn rank(self) -> u32 { return self.0/8 + 1; }

    pub fn file(self) -> u32 { return 8 - self.0 % 8; }

    pub fn from_rank_file(rank: u32, file: u32) -> Option<Square> {
        let idx = (rank - 1) * 8 + file;
        if idx < 64 {
            return Some(Square::new(idx));
        } else {
            return None;
        }
    }

    pub fn from_algebraic(alg: &str) -> Option<Square> {
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

    pub fn to_algebraic(&self) -> String {
        let mut alg_str: String = String::new();

        //TODO: create rank/file newtype
        let file = match self.file() {
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            _ => 'X'
        };

        use std::char::from_digit;
        let rank = from_digit(self.rank(), 10).unwrap();

        alg_str.push(file);
        alg_str.push(rank);

        return alg_str;
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum PieceType {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6
}

impl PieceType {
    pub fn all() -> Iter<'static, PieceType> {
        use self::PieceType::*;
        static PIECETYPES: [PieceType;  6] = [Pawn, Knight, Bishop, Rook, Queen, King];
        PIECETYPES.into_iter()
    }

    pub fn is_slider(self) -> bool {
        use self::PieceType::*;
        return match self {
            Pawn   => false,
            Knight => false,
            Bishop => true,
            Rook   => true,
            Queen  => true,
            King   => false
        }
    }

    pub fn from_bits(bits: u32) -> PieceType {
        use self::PieceType::*;
        match bits {
            1 => Pawn,
            2 => Knight,
            3 => Bishop,
            4 => Rook,
            5 => Queen,
            6 => King,
            _ => panic!("Invalid bits passed to PieceType::from_bits!")
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color { White, Black }

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

impl Piece {
    pub fn new(c: Color, p: PieceType) -> Piece {
        Piece {
            ptype: p,
            color: c
        }
    }
}

bitflags! {
    pub struct CastlingRights: u8 {
        const WHITE_KINGSIDE  = 0b0001;
        const WHITE_QUEENSIDE = 0b0010;
        const BLACK_QUEENSIDE = 0b0100;
        const BLACK_KINGSIDE  = 0b1000;
    }
}

impl CastlingRights {
    pub fn flip_color(self) -> CastlingRights {
        CastlingRights::from_bits(self.bits().reverse_bits() >> 4).unwrap()
    }
}
