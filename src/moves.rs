use core::*;

use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move(u32);

//TODO: create newtype for MoveFlag
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
    pub fn new_quiet( from: Square
                    , to: Square
                    , flag: u32
                    , moved_piece: PieceType
                    ) -> Move
    {
        return Move(
              ((moved_piece as u32) << 16)
            | (flag << 12)
            | (from.unwrap() << 6)
            | to.unwrap()
        );
    }

    pub fn new_capture( from: Square
                      , to: Square
                      , flag: u32
                      , moved_piece: PieceType
                      , captured_piece: PieceType
                      ) -> Move
    {
        return Move(
              ((captured_piece as u32) << 19)
            | ((moved_piece as u32) << 16)
            | (flag << 12)
            | (from.unwrap() << 6)
            | to.unwrap()
        );
    }


    #[allow(dead_code)]
    pub fn new_capture_detailed( from: Square
                      , to: Square
                      , flag: u32
                      , moved_piece: PieceType
                      , captured_piece: PieceType
                      ) -> Move
    {
        let a = ((captured_piece as u32) << 19);
        let b = ((moved_piece as u32) << 16);
        let c = ((flag) << 12);
        let d = ((from.unwrap()) << 6);
        let e = (to.unwrap());

        println!("{}", format!("{:32b}", a));
        println!("{}", format!("{:32b}", b));
        println!("{}", format!("{:32b}", c));
        println!("{}", format!("{:32b}", d));
        println!("{}", format!("{:32b}", e));
        println!("");

        Move( a | b | c | d | e )
    }

    pub fn to(&self) -> Square {
        return Square::new(self.0 & 0x3f);
    }

    pub fn from(&self) -> Square {
        return Square::new((self.0 >> 6) & 0x3f);
    }

    pub fn flag(&self) -> u32 {
        return (self.0 >> 12) & 0xf;
    }

    pub fn is_capture(&self) -> bool {
        return self.flag() & 0b0100 != 0;
    }

    pub fn is_promotion(&self) -> bool {
        return self.flag() & 0b1000 != 0;
    }

    pub fn moved_piece(&self) -> PieceType {
        return PieceType::from_bits((self.0 >> 16) & 0x7);
    }

    pub fn captured_piece(&self) -> Option<PieceType> {
        return match (self.0 >> 19) & 0x7 {
            0 => None,
            bits => Some(PieceType::from_bits(bits))
        }
    }

    pub fn unwrap(&self) -> u32 {
        self.0
    }

    pub fn wrap(val: u32) -> Move {
        Move(val)
    }

    pub fn null() -> Move {
        Move(0x0)
    }

    pub fn is_null(&self) -> bool {
        self.0 == 0x0
    }

    pub fn to_uci_str(&self) -> String {
        //TODO: add promotion type
        format!("{}{}", self.from().to_algebraic(), self.to().to_algebraic())
    }
}

#[cfg(test)]
mod test {
    use moves::*;
    use rand::{thread_rng, Rng};

    fn random_flag() -> u32 {
        match thread_rng().gen_range(0,14) {
            0 => QUIET_FLAG,
            1 => DOUBLE_PAWN_PUSH_FLAG,
            2 => KING_CASTLE_FLAG,
            3 => QUEEN_CASTLE_FLAG,
            4 => CAPTURE_FLAG,
            5 => EP_CAPTURE_FLAG,
            6 => KNIGHT_PROMO_FLAG,
            7 => BISHOP_PROMO_FLAG,
            8 => ROOK_PROMO_FLAG,
            9 => QUEEN_PROMO_FLAG,
            10 => KNIGHT_PROMO_CAPTURE_FLAG,
            11 => BISHOP_PROMO_CAPTURE_FLAG,
            12 => ROOK_PROMO_CAPTURE_FLAG,
            13 => QUEEN_PROMO_CAPTURE_FLAG,
            _ => panic!("Invalid random flag generation!")
        }
    }

    fn random_square() -> Square {
        Square::new(thread_rng().gen_range(0,64))
    }

    fn random_ptype() -> PieceType {
        PieceType::from_bits(thread_rng().gen_range(1,7))
    }

    #[test]
    fn bit_conversion() {
        // these are not legal/sensible moves, just testing bitwise wrap/unwrap consistency
        for _ in 0 .. 10000000 {
            let from = random_square();
            let to = random_square();
            let flag = random_flag();
            let move_ptype = random_ptype();
            let captured_ptype = random_ptype();

            let qm = Move::new_quiet(from, to, flag, move_ptype);
            let cm = Move::new_capture(from, to, flag, move_ptype, captured_ptype);

            assert!(qm.from() == from);
            assert!(qm.to() == to);
            assert!(qm.flag() == flag);
            assert!(qm.moved_piece() == move_ptype);

            assert!(cm.from() == from);
            assert!(cm.to() == to);
            assert!(cm.flag() == flag);
            assert!(cm.moved_piece() == move_ptype);
            assert!(cm.captured_piece().unwrap() == captured_ptype);
        }
    }
}
