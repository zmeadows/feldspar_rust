use core::*;

use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::BitXor;
use std::ops::BitXorAssign;
use std::ops::Not;
use std::ops::Shl;
use std::ops::Shr;

use std::simd::{u64x4};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const fn new(bb: u64) -> Bitboard { return Bitboard(bb); }

    pub fn bitscan_forward(self) -> Square { return Square::new(self.0.trailing_zeros()); }
    pub fn bitscan_reverse(self) -> Square { return Square::new(63 - self.0.leading_zeros()); }
    pub fn nonempty(self) -> bool { return self.0 != 0; }
    pub fn empty(self) -> bool { return self.0 == 0; }

    pub fn all_set() -> Bitboard { Bitboard::new(u64::max_value()) }
    pub fn none_set() -> Bitboard { Bitboard::new(0) }

    pub fn shifted_up(self) -> Bitboard { return Bitboard(self.0 << 8); }
    pub fn shifted_down(self) -> Bitboard { return Bitboard(self.0 >> 8); }

    pub fn population(self) -> u32 { self.0.count_ones() }

    pub fn unwrap(self) -> u64 { self.0 }
    pub fn split(self) -> BitboardSplitter { BitboardSplitter { bits: self } }

    pub fn east_one (b: Bitboard) -> Bitboard {return (b << 1) & NOTAFILE;}
    pub fn northeast_one (b: Bitboard) -> Bitboard {return (b << 9) & NOTAFILE;}
    pub fn southeast_one (b: Bitboard) -> Bitboard {return (b >> 7) & NOTAFILE;}
    pub fn west_one (b: Bitboard) -> Bitboard {return (b >> 1) & NOTHFILE;}
    pub fn southwest_one (b: Bitboard) -> Bitboard {return (b >> 9) & NOTHFILE;}
    pub fn northwest_one (b: Bitboard) -> Bitboard {return (b << 7) & NOTHFILE;}
    pub fn south_one (b: Bitboard) -> Bitboard {return  b >> 8;}
    pub fn north_one (b: Bitboard) -> Bitboard {return  b << 8;}

    pub fn flip_color(self) -> Bitboard {
        return Bitboard(self.0.reverse_bits());
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAnd<QuadBitboard> for Bitboard {
    type Output = QuadBitboard;
    fn bitand(self, rhs: QuadBitboard) -> QuadBitboard {
        QuadBitboard(self.0 & rhs.0)
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

impl Shl<usize> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: usize) -> Bitboard {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<usize> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: usize) -> Bitboard {
        Bitboard(self.0 >> rhs)
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

pub struct BitboardSplitter {
    bits: Bitboard
}

impl Iterator for BitboardSplitter {
    type Item = Bitboard;

    fn next(&mut self) -> Option<Bitboard> {
        if self.bits.nonempty() {
            let old_bits = self.bits;
            self.bits &= Bitboard::new(self.bits.0 - 1);
            return Some(self.bits ^ old_bits);
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct QuadBitboard(u64x4);

impl QuadBitboard {
    pub const fn new(a: u64, b: u64, c: u64, d: u64) -> QuadBitboard {
        QuadBitboard(u64x4::new(a,b,c,d))
    }

    pub const fn splat(a: u64) -> QuadBitboard {
        QuadBitboard(u64x4::splat(a))
    }

    pub fn east_one (b: QuadBitboard) -> QuadBitboard {return (b << 1) & NOTAFILE;}
    pub fn northeast_one (b: QuadBitboard) -> QuadBitboard {return (b << 9) & NOTAFILE;}
    pub fn southeast_one (b: QuadBitboard) -> QuadBitboard {return (b >> 7) & NOTAFILE;}
    pub fn west_one (b: QuadBitboard) -> QuadBitboard {return (b >> 1) & NOTHFILE;}
    pub fn southwest_one (b: QuadBitboard) -> QuadBitboard {return (b >> 9) & NOTHFILE;}
    pub fn northwest_one (b: QuadBitboard) -> QuadBitboard {return (b << 7) & NOTHFILE;}
    pub fn south_one (b: QuadBitboard) -> QuadBitboard {return  b >> 8;}
    pub fn north_one (b: QuadBitboard) -> QuadBitboard {return  b << 8;}

    pub fn extract(&self, idx: usize) -> Bitboard { Bitboard::new(self.0.extract(idx)) }

    pub fn or(&self) -> Bitboard { Bitboard::new(self.0.or()) }
}

impl BitAnd for QuadBitboard {
    type Output = QuadBitboard;
    fn bitand(self, rhs: QuadBitboard) -> QuadBitboard {
        QuadBitboard(self.0 & rhs.0)
    }
}

impl BitAnd<Bitboard> for QuadBitboard {
    type Output = QuadBitboard;
    fn bitand(self, rhs: Bitboard) -> QuadBitboard {
        QuadBitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for QuadBitboard {
    fn bitand_assign(&mut self, rhs: QuadBitboard) {
        self.0 &= rhs.0;
    }
}

impl BitAndAssign<Bitboard> for QuadBitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0;
    }
}

impl BitOr for QuadBitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        QuadBitboard(self.0 | rhs.0)
    }
}

impl BitOr<Bitboard> for QuadBitboard {
    type Output = Self;
    fn bitor(self, rhs: Bitboard) -> Self {
        QuadBitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for QuadBitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitOrAssign<Bitboard> for QuadBitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitXor for QuadBitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        QuadBitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for QuadBitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for QuadBitboard {
    type Output = QuadBitboard;

    fn not(self) -> QuadBitboard {
        return QuadBitboard(!self.0);
    }
}

impl Shl<usize> for QuadBitboard {
    type Output = Self;
    fn shl(self, rhs: usize) -> QuadBitboard {
        QuadBitboard(self.0 << rhs)
    }
}

impl Shr<usize> for QuadBitboard {
    type Output = Self;
    fn shr(self, rhs: usize) -> QuadBitboard {
        QuadBitboard(self.0 >> rhs)
    }
}

pub const NOTAFILE: Bitboard = Bitboard::new(0xfefefefefefefefe);
pub const NOTHFILE: Bitboard = Bitboard::new(0x7f7f7f7f7f7f7f7f);
pub const QUAD_NOTAFILE: QuadBitboard = QuadBitboard::splat(0xfefefefefefefefe);
pub const QUAD_NOTHFILE: QuadBitboard = QuadBitboard::splat(0x7f7f7f7f7f7f7f7f);
