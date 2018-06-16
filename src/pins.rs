use core::*;
use bitboard::*;
use moves::*;
use board::*;
use tables::*;
use game::*;

#[derive(Clone, Copy)]
pub struct PinFinder {
    diag_pin_map: [Bitboard; 64],
    nondiag_pin_map: [Bitboard; 64],
    pinned_diagonally: Bitboard,
    pinned_nondiagonally: Bitboard
}

impl PinFinder {
    pub fn new() -> PinFinder {
        PinFinder {
            diag_pin_map: [Bitboard::new(0); 64],
            nondiag_pin_map: [Bitboard::new(0); 64],
            pinned_diagonally: Bitboard::new(0),
            pinned_nondiagonally: Bitboard::new(0)
        }
    }

    pub fn update(&mut self, moving_color: Color, board: &Board) {
        use PieceType::*;

        self.pinned_diagonally = Bitboard::new(0);
        self.pinned_nondiagonally = Bitboard::new(0);

        let opponent_color = !moving_color;
        let occupied_squares = board.occupied();
        let friendly_pieces = board.occupied_by(moving_color);
        let king_square = board.get_king_square(moving_color);

        let op_rq = board.get_pieces(opponent_color, Rook) | board.get_pieces(opponent_color, Queen);
        let mut pinner = xray_rook_attacks(occupied_squares, friendly_pieces, king_square) & op_rq;
        for pinner_square in pinner {
            let connecting_bits = ray_between_squares(king_square, pinner_square);
            let pinned_bit = connecting_bits & friendly_pieces;
            self.nondiag_pin_map[pinned_bit.bitscan_forward().idx()] = connecting_bits;
            assert!(pinned_bit.population() == 1);
            self.pinned_nondiagonally |= pinned_bit;
        }

        let op_bq = board.get_pieces(opponent_color, Bishop) | board.get_pieces(opponent_color, Queen);
        pinner = xray_bishop_attacks(occupied_squares, friendly_pieces, king_square) & op_bq;
        for pinner_square in pinner {
            let connecting_bits = ray_between_squares(king_square, pinner_square);
            let pinned_bit = connecting_bits & friendly_pieces;
            self.diag_pin_map[pinned_bit.bitscan_forward().idx()] = connecting_bits;
            assert!(pinned_bit.population() == 1);
            self.pinned_diagonally |= pinned_bit;
        }
    }

    pub fn pinned_diagonally(&self) -> Bitboard {
        self.pinned_diagonally
    }

    pub fn pinned_nondiagonally(&self) -> Bitboard {
        self.pinned_nondiagonally
    }

    pub fn pinned(&self) -> Bitboard {
        self.pinned_diagonally | self.pinned_nondiagonally
    }

    pub fn diagonal_constraint(&self, sq: Square) -> Bitboard {
        self.diag_pin_map[sq.idx()]
    }

    pub fn nondiagonal_constraint(&self, sq: Square) -> Bitboard {
        self.nondiag_pin_map[sq.idx()]
    }
}
