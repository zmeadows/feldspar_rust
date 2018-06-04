use core::*;

use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::BitXor;
use std::ops::BitXorAssign;
use std::ops::Not;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const fn new(bb: u64) -> Bitboard { return Bitboard(bb); }

    //TODO: test performance of alternative: De Bruijn Multiplication
    pub fn bitscan_forward(self) -> Square { return Square::new(self.0.trailing_zeros()); }
    pub fn bitscan_reverse(self) -> Square { return Square::new(63 - self.0.leading_zeros()); }
    pub fn nonempty(self) -> bool { return self.0 != 0; }
    pub fn empty(self) -> bool { return self.0 == 0; }

    pub fn shifted_left(self) -> Bitboard { return Bitboard(self.0 >> 1); }
    pub fn shifted_right(self) -> Bitboard { return Bitboard(self.0 << 1); }
    pub fn shifted_down(self) -> Bitboard { return Bitboard(self.0 >> 8); }
    pub fn shifted_up(self) -> Bitboard { return Bitboard(self.0 << 8); }

    pub fn population(self) -> u32 { self.0.count_ones() }

    pub fn unwrap(self) -> u64 { return self.0; }
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
            self.bits &= Bitboard::new(self.bits.0 - 1);
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

impl Square {
    pub fn bitrep(self) -> Bitboard { Bitboard(1 << self.unwrap()) }
}

