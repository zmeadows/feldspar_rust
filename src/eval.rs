use board::*;
use core::*;
use moves::*;

pub fn recompute_score(board: &Board) -> Score {
    use PieceType::*; use Color::*;

    let material_score = |ptype: PieceType| {
        let diff = board.get_pieces(White, ptype).population() as i32
                 - board.get_pieces(Black, ptype).population() as i32;

        let value: i32 = material_value(ptype);
        return value * diff;
    };

    let piece_square_score = |ptype: PieceType| {
        let mut diff = 0;

        for color in [White, Black].iter() {
            for sq in board.get_pieces(*color, ptype) {
                diff += piece_square_value(*color, ptype, sq);
            }
        }

        return diff;
    };

    let mut s: i32 = 0;
    for ptype in PieceType::all() {
        s += material_score(*ptype);
        s += piece_square_score(*ptype);
    }

    return Score::new(s);
}

impl Score {
    pub fn add_piece(&mut self, piece: Piece, sq: Square) {
        match piece.color {
            Color::White => self.val += material_value(piece.ptype),
            Color::Black => self.val -= material_value(piece.ptype),
        }

        self.val += piece_square_value(piece.color, piece.ptype, sq);
    }

    pub fn remove_piece(&mut self, piece: Piece, sq: Square) {
        match piece.color {
            Color::White => self.val -= material_value(piece.ptype),
            Color::Black => self.val += material_value(piece.ptype),
        }

        self.val -= piece_square_value(piece.color, piece.ptype, sq);
    }
}

fn material_value(ptype: PieceType) -> i32 {
    match ptype {
        PieceType::Pawn   => 100,
        PieceType::Knight => 320,
        PieceType::Bishop => 330,
        PieceType::Rook   => 500,
        PieceType::Queen  => 900,
        PieceType::King   => 20000
    }
}

fn piece_square_value(color: Color, ptype: PieceType, sq: Square) -> i32 {
    let idx = match color {
        Color::White => 63 - sq.idx(),
        Color::Black => 8 * (sq.idx() / 8) + 7 - sq.idx() % 8 as usize
    };

    let sf = match color {
        Color::White => 1,
        Color::Black => -1
    };

    return sf * match ptype {
        PieceType::Pawn   => unsafe { *PAWN_TABLE.get_unchecked(idx) },
        PieceType::Knight => unsafe { *KNIGHT_TABLE.get_unchecked(idx) },
        PieceType::Bishop => unsafe { *BISHOP_TABLE.get_unchecked(idx) },
        PieceType::Rook   => unsafe { *ROOK_TABLE.get_unchecked(idx) },
        PieceType::Queen  => unsafe { *QUEEN_TABLE.get_unchecked(idx) },
        PieceType::King   => unsafe { *KING_TABLE.get_unchecked(idx) }
    };
}

const PAWN_TABLE: [i32; 64] =
[ 
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    5, 10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

const KNIGHT_TABLE: [i32; 64] =
[ 
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50
];

const BISHOP_TABLE: [i32; 64] =
[ 
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50
];

const ROOK_TABLE: [i32; 64] =
[ 
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];

const QUEEN_TABLE: [i32; 64] =
[ 
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -5,  0,  5,  5,  5,  5,  0, -5,
    0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

const KING_TABLE: [i32; 64] =
[ 
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    20, 20,  0,  0,  0,  0, 20, 20,
    20, 30, 10,  0,  0, 10, 30, 20
];

