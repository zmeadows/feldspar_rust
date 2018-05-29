use core::*;
use tables::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    pieces: [Bitboard;12],
    occupied: [Bitboard;2],
    unoccupied: Bitboard,
}

impl Board {
    pub fn starting_position() -> Board {
        return Board {
            pieces: [ Bitboard::new(0x000000000000FF00), Bitboard::new(71776119061217280),
                      Bitboard::new(0x0000000000000042), Bitboard::new(0x4200000000000000),
                      Bitboard::new(0x0000000000000024), Bitboard::new(0x2400000000000000),
                      Bitboard::new(0x0000000000000081), Bitboard::new(0x8100000000000000),
                      Bitboard::new(0x0000000000000008), Bitboard::new(0x0800000000000000),
                      Bitboard::new(0x0000000000000010), Bitboard::new(0x1000000000000000)],
            occupied: [ Bitboard::new(0x000000000000FFFF), Bitboard::new(0xFFFF000000000000) ],
            unoccupied: Bitboard::new(0x0000FFFFFFFF0000)
        }
    }

    pub fn get_pieces(&self, color: Color, ptype: PieceType) -> Bitboard {
        let idx = 2 * ptype as usize + color as usize;
        return self.pieces[idx];
    }

    pub fn occupied_by(&self, color: Color) -> Bitboard {
        return self.occupied[color as usize];
    }

    pub fn occupied(&self) -> Bitboard {
        return self.occupied[0] | self.occupied[1];
    }

    pub fn unoccupied(&self) -> Bitboard {
        return self.unoccupied;
    }

    // pub fn update(&mut self, color: Color, ptype: PieceType, newBB: Bitboard) {
    //     let idx = 2 * (ptype as usize) + (color as usize);
    //     self.0[idx] = newBB;
    // }

    pub fn color_at(&self, sq: Square) -> Option<Color> {
        let bit = sq.bitrep();

        if (bit & self.occupied()).empty() { return None }

        if (bit & self.occupied_by(Color::White)).nonempty() {
            return Some(Color::White);
        } else {
            return Some(Color::Black);
        };
    }

    pub fn piece_at(&self, sq: Square) -> Option<Piece> {

        match self.color_at(sq) {
            None => return None,
            Some(col) => {
                let bit = sq.bitrep();
                for pt in PieceType::all() {
                    if (bit & self.get_pieces(col, *pt)).nonempty() {
                        return Some(Piece { ptype: *pt, color: col });
                    }
                }
            }
        }

        //TODO: Warning
        return None;
    }

    pub fn get_king_square(&self, color: Color) -> Square {
        self.get_pieces(color, PieceType::King).bitscan_forward()
    }

    pub fn is_square_attacked_by(&self, square: Square, color: Color) -> bool {

        use PieceType::*;

        if (PAWN_ATTACKS[!color as usize][square.idx()] & self.get_pieces(color, Pawn)).nonempty() {
            return true;
        } else if (KNIGHT_TABLE[square.idx()] & self.get_pieces(color, Knight)).nonempty() {
            return true;
        } else if (KING_TABLE[square.idx()] & self.get_pieces(color, King)).nonempty() {
            return true;
        }

        let occupied = self.occupied();

        let bishops_queens = self.get_pieces(color, Queen) | self.get_pieces(color, Bishop);
        if (get_bishop_rays(square, occupied) & bishops_queens).nonempty() {
            return true;
        }

        let rooks_queens = self.get_pieces(color, Queen) | self.get_pieces(color, Rook);
        if (get_rook_rays(square, occupied) & rooks_queens).nonempty() {
            return true;
        }

        return false;
    }
}
