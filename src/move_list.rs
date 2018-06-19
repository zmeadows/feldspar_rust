use core::*;
use moves::*;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub struct MoveList {
    moves: [Move; 110],
    count: usize
}

impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            //TODO: encode NULL move in Move type somehow
            moves: [Move::null(); 110],
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

    #[allow(dead_code)]
    pub fn at(&self, idx: usize) -> Move { return self.moves[idx]; }

    pub fn sort(&mut self) {
        self.moves[..self.count].sort_by(|m1, m2| {
            if m1.is_capture() && !m2.is_capture() {
                return Ordering::Less;
            } else if !m1.is_capture() && m2.is_capture() {
                return Ordering::Greater;
            } else if m1.is_capture() && m2.is_capture() { 
                let p1 = m1.captured_piece().unwrap();
                let m1 = m1.moved_piece();
                let p2 = m2.captured_piece().unwrap();
                let m2 = m2.moved_piece();

                let d1 = p1 as i32 - m1 as i32;
                let d2 = p2 as i32 - m2 as i32;

                if d1 > d2 {
                    return Ordering::Less;
                } else if d2 > d1 {
                    return Ordering::Greater;
                } else {
                    return Ordering::Equal;
                }
            } else {
                return Ordering::Equal;
            }
        });
    }

}

#[derive(Clone, Copy)]
pub struct MoveListIterator<'a> {
    move_list: &'a MoveList,
    current: usize
}

//TODO: IntoIter for MoveList?

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

