use core::*;
use bitboard::*;
use tables::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Board {
    pieces: [Bitboard;12],
    occupied: [Bitboard;2]
}

impl Board {
    #[allow(dead_code)]
    pub fn starting_position() -> Board {
        return Board {
            pieces: [ Bitboard::new(0x000000000000FF00), Bitboard::new(71776119061217280),
                      Bitboard::new(0x0000000000000042), Bitboard::new(0x4200000000000000),
                      Bitboard::new(0x0000000000000024), Bitboard::new(0x2400000000000000),
                      Bitboard::new(0x0000000000000081), Bitboard::new(0x8100000000000000),
                      Bitboard::new(0x0000000000000008), Bitboard::new(0x0800000000000000),
                      Bitboard::new(0x0000000000000010), Bitboard::new(0x1000000000000000)],
            occupied: [ Bitboard::new(0x000000000000FFFF), Bitboard::new(0xFFFF000000000000) ]
        }
    }

    pub fn empty_position() -> Board {
        return Board {
            pieces: [ Bitboard::new(0), Bitboard::new(0),
                      Bitboard::new(0), Bitboard::new(0),
                      Bitboard::new(0), Bitboard::new(0),
                      Bitboard::new(0), Bitboard::new(0),
                      Bitboard::new(0), Bitboard::new(0),
                      Bitboard::new(0), Bitboard::new(0)],
            occupied: [ Bitboard::new(0), Bitboard::new(0) ]
        }
    }

    pub fn get_pieces(&self, color: Color, ptype: PieceType) -> Bitboard {
        let idx = 2 * (ptype as usize - 1) + color as usize;
        return unsafe { *self.pieces.get_unchecked(idx) };
    }

    pub fn get_pieces_mut(&mut self, color: Color, ptype: PieceType) -> &mut Bitboard {
        let idx = 2 * (ptype as usize - 1) + color as usize;
        return unsafe { self.pieces.get_unchecked_mut(idx) };
    }

    pub fn set_piece_bit(&mut self, color: Color, ptype: PieceType, square: Square) {
        let bit = square.bitrep();
        *self.get_pieces_mut(color, ptype) |= bit;
        self.occupied[color as usize] |= bit;
    }

    pub fn occupied_by(&self, color: Color) -> Bitboard {
        return self.occupied[color as usize];
    }

    pub fn occupied_by_mut(&mut self, color: Color) -> &mut Bitboard {
        return &mut self.occupied[color as usize];
    }

    pub fn occupied(&self) -> Bitboard {
        return self.occupied[0] | self.occupied[1];
    }

    pub fn unoccupied(&self) -> Bitboard {
        return !self.occupied();
    }

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
        let bit = sq.bitrep();

        match self.color_at(sq) {
            None => return None,
            Some(col) => {
                for pt in PieceType::all() {
                    if (bit & self.get_pieces(col, *pt)).nonempty() {
                        return Some(Piece { ptype: *pt, color: col });
                    }
                }
            }
        }

        return None;
    }

    //OPTIMIZE: keep king squares in Game struct?
    pub fn get_king_square(&self, color: Color) -> Square {
        let k = self.get_pieces(color, PieceType::King);
        k.bitscan_forward()
    }

    pub fn attackers(&self, square: Square, color: Color) -> Bitboard {
        use PieceType::*;

        let mut attackers: Bitboard = Bitboard::new(0);
        let idx = square.idx();

        unsafe {
            attackers |= *PAWN_ATTACKS.get_unchecked(!color as usize).get_unchecked(idx) & self.get_pieces(color, Pawn);
            attackers |= *KNIGHT_TABLE.get_unchecked(idx) & self.get_pieces(color, Knight);
            attackers |= *KING_TABLE.get_unchecked(idx) & self.get_pieces(color, King);
        }

        let occupied = self.occupied();

        let bishops_queens = self.get_pieces(color, Queen) | self.get_pieces(color, Bishop);
        attackers |= get_bishop_rays(square, occupied) & bishops_queens;

        let rooks_queens = self.get_pieces(color, Queen) | self.get_pieces(color, Rook);
        attackers |= get_rook_rays(square, occupied) & rooks_queens;

        return attackers;
    }

    //OPTIMIZE: do flood fill instead of generating attacks for individual pieces
    // since it doesn't matter which piece is attacking where.
    pub fn attacked(&self, attacking_color: Color, remove_king: bool) -> Bitboard {
        use PieceType::*;

        let defending_color = !attacking_color;
        let mut attacked: Bitboard = Bitboard::new(0);

        let defending_pieces = if remove_king {
            self.occupied_by(defending_color) & !self.get_king_square(defending_color).bitrep()
        } else {
            self.occupied_by(defending_color)
        };

        let attacking_pieces = self.occupied_by(attacking_color);
        let all_pieces = defending_pieces | attacking_pieces;

        unsafe {
            for from in self.get_pieces(attacking_color, Pawn) {
                attacked |= *PAWN_ATTACKS.get_unchecked(attacking_color as usize).get_unchecked(from.idx());
            }

            for from in self.get_pieces(attacking_color, Knight) {
                attacked |= *KNIGHT_TABLE.get_unchecked(from.idx());
            }

            attacked |= *KING_TABLE.get_unchecked(self.get_king_square(attacking_color).idx());
        }

        for from in self.get_pieces(attacking_color, Bishop) {
            attacked |= get_bishop_rays(from, all_pieces);
        }

        for from in self.get_pieces(attacking_color, Rook) {
            attacked |= get_rook_rays(from, all_pieces);
        }

        for from in self.get_pieces(attacking_color, Queen) {
            attacked |= get_queen_rays(from, all_pieces);
        }


        return attacked;
    }
}
