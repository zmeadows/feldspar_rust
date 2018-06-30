use bitboard::*;
use board::*;
use core::*;
use moves::*;
use moves::*;
use tables::*;
use game::*;
use eval::*;

use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Hash(u64);

static mut piece_keys: [[u64;64]; 12] = [ [0;64]; 12 ];
static mut black_to_move_key: u64 = 0;
static mut castle_keys: [u64; 16] = [0; 16];
static mut ep_keys: [u64; 8] = [0; 8];

impl Hash {
    pub fn change_piece(&mut self, color: Color, piece_type: PieceType, square: Square) {
        unsafe {
            self.0 ^= *piece_keys.get_unchecked(2 * (piece_type as usize - 1) + (color as usize)).get_unchecked(square.idx());
        }
    }

    pub fn update_black_to_move(&mut self) {
        unsafe {
            self.0 ^= black_to_move_key;
        }
    }

    pub fn update_castling_rights(&mut self, rights: CastlingRights) {
        unsafe {
            self.0 ^= *castle_keys.get_unchecked(rights.bits() as usize);
        }
    }

    pub fn modify_ep_square(&mut self, square: Square) {
        unsafe {
            self.0 ^= *ep_keys.get_unchecked(square.file() as usize - 1);
        }
    }

    pub fn unwrap(self) -> u64 { return self.0 }

    pub fn empty() -> Hash {
        Hash(0)
    }

    pub fn new(game: &Game) -> Hash {
        let mut hash = Hash::empty();

        unsafe {
            for color in [Color::White, Color::Black].iter() {
                for piece_type in PieceType::all() {
                    for square in game.board.get_pieces(*color, *piece_type) {
                        hash.change_piece(*color, *piece_type, square);
                    }
                }
            }

            hash.0 ^= castle_keys[game.castling_rights.bits() as usize];

            if game.to_move == Color::Black {
                hash.0 ^= black_to_move_key;
            }

            match game.ep_square {
                Some(square) => hash.0 ^= ep_keys[square.file() as usize - 1],
                None => {}
            }
        }

        return hash;
    }
}

pub fn init_zobrist_hashing() {
    unsafe {
        for i in 0 .. 12 {
            for j in 0 .. 64 {
                piece_keys[i][j] = rand::random::<u64>();
            }
        }

        black_to_move_key = rand::random::<u64>();

        for i in 0 .. 16 {
            castle_keys[i] = rand::random::<u64>();
        }

        for i in 0 .. 8 {
            ep_keys[i] = rand::random::<u64>();
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EntryData(u64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NodeType {
    PV = 0,
    All = 1,
    Cut = 2
}

impl EntryData {
    pub fn new( best_move: Move
              , score: Score
              , depth: u8
              , node_type: NodeType
              , age: u8) -> EntryData
    {
        EntryData(
              (age as u64) << 56
            | (depth as u64) << 50
            | (node_type as u64) << 48
            | (score.store_u16() as u64) << 32
            | best_move.unwrap() as u64
        )
    }

    pub fn empty() -> EntryData {
        EntryData(0)
    }

    pub fn unwrap(self) -> u64 {
        self.0
    }

    pub fn age(self) -> u8 {
        ((self.0 >> 56) & 0xff) as u8
    }

    pub fn best_move(self) -> Move {
        Move::wrap( (self.0 & 0xffffffff) as u32 )
    }

    pub fn depth(self) -> u8 {
        ((self.0 >> 50) & 0x3f) as u8
    }

    pub fn node_type(self) -> NodeType {
        match (self.0 >> 48) & 3 {
            0 => NodeType::PV,
            1 => NodeType::All,
            2 => NodeType::Cut,
            x => panic!("Invalid conversion from bits to NodeType: {}", x)
        }
    }

    pub fn score(self) -> Score {
        Score::unstore_u16( ((self.0 >> 32) & 0xffff) as u16 )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Key(u64);

impl Key {
    fn new(hash: Hash, entry: EntryData) -> Key {
        Key(hash.unwrap() ^ entry.unwrap())
    }

    fn empty() -> Key {
        Key(0)
    }

    fn unwrap(self) -> u64 { self.0 }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableEntry {
    key: Key,
    entry: EntryData
}

impl TableEntry {
    pub fn new(new_hash: Hash, new_entry: EntryData) -> TableEntry {
        TableEntry {
            key: Key::new(new_hash, new_entry),
            entry: new_entry
        }
    }

    pub fn empty() -> TableEntry {
        TableEntry {
            key: Key::empty(),
            entry: EntryData::empty()
        }
    }
}

#[derive(Debug, Clone)]
pub struct TranspositionTable {
    entries: Vec<TableEntry>
}

impl TranspositionTable {
    pub fn new(count: usize) -> TranspositionTable {
        TranspositionTable {
            entries: vec![TableEntry::empty(); count]
        }
    }

    pub fn probe(&self, hash: Hash) -> Option<EntryData> {
        let idx = (hash.unwrap() % self.entries.len() as u64) as usize;

        let probed_entry = unsafe { self.entries.get_unchecked(idx) };

        if (probed_entry.key.unwrap() ^ probed_entry.entry.unwrap() == hash.unwrap()) {
            return Some(probed_entry.entry);
        } else {
            return None;
        }
    }

    pub fn update(&mut self, hash: Hash, new_entry: EntryData) {
        let idx = (hash.unwrap() % self.entries.len() as u64) as usize;
        let new_table_entry = TableEntry::new(hash, new_entry);
        unsafe {
            *self.entries.get_unchecked_mut(idx) = new_table_entry;
        }
    }

    pub fn reset(&mut self) {
        for x in self.entries.iter_mut() {
            x.key = Key::empty();
            x.entry = EntryData::empty();
        }
    }

    pub fn get_pv(&self, mut game: Game, mut max_length: usize) -> Vec<EntryData> {
        let mut variation = Vec::new();

        while max_length != 0 {
            match self.probe(game.hash) {
                None => break,
                Some(tentry) => {
                    match tentry.node_type() {
                        NodeType::PV => variation.push(tentry),
                        _ => break
                    }
                    let best_move = tentry.best_move();
                    game.make_move(best_move);
                    max_length -= 1;
                }
            }
        }

        return variation;
    }
}

#[cfg(test)]
mod test {
    use zobrist::*;
    use rand::{thread_rng, Rng};

    fn random_node_type() -> NodeType {
        match thread_rng().gen_range(0,3) {
            0 => NodeType::PV,
            1 => NodeType::All,
            2 => NodeType::Cut,
            _ => panic!("Invalid random NodeType generation!")
        }
    }

    #[test]
    fn bit_conversion() {
        // these are not sensible entries, just testing bitwise wrap/unwrap consistency
        let mut entry_data = EntryData::empty();
        for _ in 0 .. 1000 {
            let random_move = Move::wrap(rand::random::<u32>());
            let random_score = Score::new(rand::random::<i16>());
            let random_depth = thread_rng().gen_range(0,0x3f);
            let random_node_type = random_node_type();
            let random_age = thread_rng().gen_range(0,0xff) as u8;

            entry_data = EntryData::new(random_move, random_score, random_depth, random_node_type, random_age);

            assert!(entry_data.best_move() == random_move);
            assert!(entry_data.score() == random_score);
            assert!(entry_data.age() == random_age);
            assert!(entry_data.depth() == random_depth as u8);
            assert!(entry_data.node_type() == random_node_type);
        }
    }
}
