use core::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move(u32);

/* 4-bit sequences describing move type */
pub const QUIET_FLAG                : u32 = 0b0000;
pub const DOUBLE_PAWN_PUSH_FLAG     : u32 = 0b0001;
pub const KING_CASTLE_FLAG          : u32 = 0b0010;
pub const QUEEN_CASTLE_FLAG         : u32 = 0b0011;
pub const CAPTURE_FLAG              : u32 = 0b0100;
pub const EP_CAPTURE_FLAG           : u32 = 0b0101;
pub const KNIGHT_PROMO_FLAG         : u32 = 0b1000;
pub const BISHOP_PROMO_FLAG         : u32 = 0b1001;
pub const ROOK_PROMO_FLAG           : u32 = 0b1010;
pub const QUEEN_PROMO_FLAG          : u32 = 0b1011;
pub const KNIGHT_PROMO_CAPTURE_FLAG : u32 = 0b1100;
pub const BISHOP_PROMO_CAPTURE_FLAG : u32 = 0b1101;
pub const ROOK_PROMO_CAPTURE_FLAG   : u32 = 0b1110;
pub const QUEEN_PROMO_CAPTURE_FLAG  : u32 = 0b1111;

impl Move {
    pub fn new(from: Square, to: Square, flag: u32) -> Move {
        return Move(((flag & 0xf) << 12) | ((from.unwrap() & 0x3f) << 6) | (to.unwrap() & 0x3f));
    }

    pub fn to(&self) -> Square {
        return Square::new(self.0 & 0x3f);
    }

    pub fn from(&self) -> Square {
        return Square::new((self.0 >> 6) & 0x3f);
    }

    pub fn flag(&self) -> u32 {
        return (self.0 >> 12) & 0x0f;
    }

    pub fn is_capture(&self) -> bool {
        return self.flag() & 0b0100 != 0;
    }

    pub fn is_promotion(&self) -> bool {
        return self.flag() & 0b1000 != 0;
    }
}

