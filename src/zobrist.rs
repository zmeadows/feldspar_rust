use bitboard::*;
use board::*;
use core::*;
use moves::*;
use moves::*;
use tables::*;
use game::*;

/*
struct Hash { val: u64 }

impl Hash {
    pub fn new(game: &game) -> Hash {
        Hash { val: 0x0 }
    }
}

struct EntryData {
    val: u64
}

// impl EntryData {
//     pub fn new(move: Move, score: Score, depth: u8) -> EntryData {
//         EntryData {
//             val: (move.unwrap() & <u32>::max_value() as u64) << 32)
//                  | (move.unwrap() & <u32>::max_value() as u64) << 32)
//             val: (flag & 0xf) << 12) | ((from.unwrap() & 0x3f) << 6) | (to.unwrap() & 0x3f))
//         }
//     }
// }

struct Entry {
    hash: Hash,
    data: EntryData
}

impl TableEntry {
    pub fn new() -> TableEntry {
        TableEntry {
            key: 0x0,
            data: 0x0
        }
    }
}

struct TranspositionTable {
    entries: Vec<TableEntry>
}

pub struct SharedTranspositionTable {
    entries: *mut TableEntry,
    len: usize
}

impl TranspositionTable {
    pub fn new(count: usize) -> TranspositionTable {
        TranspositionTable {
            entries: vec![TableEntry::new(); count] 
        }
    }

    pub fn share(&mut self) -> SharedTranspositionTable {
        SharedTranspositionTable {
            entries: self.entries.as_mut_ptr(),
            len: self.entries.size()
        }
    }
}

// impl SharedTranspositionTable {
//     pub fn probe(&self, 
// }
*/
