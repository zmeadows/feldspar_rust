use core::*;
use moves::*;

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
        //TODO: https://chessprogramming.wikispaces.com/Move%20Ordering
        return;
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

