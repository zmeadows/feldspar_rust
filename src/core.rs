use std::slice::Iter;
use std::str::Chars;

use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::BitXor;
use std::ops::BitXorAssign;
use std::ops::Sub;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const fn new(bb: u64) -> Bitboard { return Bitboard(bb); }

    pub fn bitscan_forward(self) -> Square { return Square(self.0.trailing_zeros()); }
    pub fn bitscan_reverse(self) -> Square { return Square(63 - self.0.leading_zeros()); }
    pub fn nonempty(self) -> bool { return self.0 != 0; }
    pub fn empty(self) -> bool { return self.0 == 0; }

    pub fn shifted_left(self) -> Bitboard { return Bitboard(self.0 >> 1); }
    pub fn shifted_right(self) -> Bitboard { return Bitboard(self.0 << 1); }
    pub fn shifted_down(self) -> Bitboard { return Bitboard(self.0 >> 8); }
    pub fn shifted_up(self) -> Bitboard { return Bitboard(self.0 << 8); }

    pub fn population(self) -> u32 { self.0.count_ones() }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        *self = Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Sub for Bitboard {
    type Output = Bitboard;

    fn sub(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 - other.0)
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Bitboard {
        return Bitboard(!self.0);
    }
}

pub struct BitboardIterator {
    bits: Bitboard
}

impl Iterator for BitboardIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        if self.bits.nonempty() {
            let sq = self.bits.bitscan_forward();
            self.bits &= self.bits - Bitboard::new(1);
            return Some(sq);
        } else {
            return None;
        }
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIterator;

    fn into_iter(self: Bitboard) -> Self::IntoIter {
        return BitboardIterator { bits: self };
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction { N, S, E, W, NE, NW, SE, SW }

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Square(u32);

impl Square {
    pub fn new(idx: u32) -> Square {
        assert!(idx < 64, "Attempted to create Square with invalid index!");
        Square(idx)
    }

    pub fn bitrep(self) -> Bitboard { Bitboard(1 << self.0) }
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

