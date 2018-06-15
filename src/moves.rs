use core::*;

use std::rc::Rc;
use std::cell::RefCell;

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

    pub fn unwrap(&self) -> u32 {
        return self.0;
    }
}

#[derive(Clone, Copy)]
pub struct MoveList {
    moves: [Move; 110],
    count: usize
}

impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            moves: [Move::new(Square::new(0),Square::new(0), QUIET_FLAG); 110],
            count: 0
        }
    }

    pub fn add(&mut self, m: Move) {
        self.moves[self.count] = m;
        self.count += 1;
    }

    pub fn clear(&mut self) {
        self.count = 0;
    }

    pub fn len(&self) -> usize { self.count }

    pub fn at(&self, idx: usize) -> Move { return self.moves[idx]; }
}

#[derive(Clone, Copy)]
pub struct MoveListIterator<'a> {
    move_list: &'a MoveList,
    current: usize
}

impl<'a> Iterator for MoveListIterator<'a> {
    type Item = &'a Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.move_list.len() {
            self.current += 1;
            return Some(&self.move_list.moves[self.current-1]);
        } else {
            return None;
        }
    }
}

impl MoveList {
    pub fn iter<'a>(&'a self) -> MoveListIterator {
        MoveListIterator {
            move_list: self,
            current: 0,
        }
    }
}

pub type MoveBuffer = Rc<RefCell<MoveList>>;

pub fn alloc_move_buffer() -> MoveBuffer {
    Rc::new(RefCell::new(MoveList::new()))
}

#[derive(Clone)]
pub struct MoveStack {
    stack: Vec<MoveBuffer>
}


impl MoveStack {
    pub fn new() -> MoveStack {
        let mut new_stack = Vec::new();

        for i in 0 .. 500 {
            new_stack.push(alloc_move_buffer());
        }

        MoveStack { stack: new_stack }
    }

    pub fn at_depth(&self, depth: usize) -> &MoveBuffer {
        &self.stack[depth - 1]
    }
}
