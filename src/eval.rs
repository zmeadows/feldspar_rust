use bitboard::*;
use board::*;
use core::*;
use moves::*;
use moves::*;
use tables::*;
use game::*;

pub type Score = f32;

pub fn simple_eval(game: &Game) -> Score {
    let mut score: Score = 0.0;

    use PieceType::*;
    use Color::*;

    let sum_pieces = |ptype: PieceType| {
        let diff = game.board.get_pieces(White, ptype).population()
                 - game.board.get_pieces(Black, ptype).population();

        let value = match ptype {
            Pawn => 100.0,
            Knight => 320.0,
            Bishop => 330.0,
            Rook => 500.0,
            Queen => 900.0,
            King => 20000.0
        };

        return value * (diff as Score);
    };

    for ptype in PieceType::all() {
        score += sum_pieces(*ptype);
    }

    return score;
}

