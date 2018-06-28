use board::*;
use core::*;
use moves::*;
use game::*;
use tables::*;
use movegen::*;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Score(i16);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Phase(u16);

impl Score {
    pub fn new(s: i16) -> Score {
        debug_assert!(s >= Score::min().0 && s <=Score::max().0);
        Score(s)
    }

    pub fn max() -> Score {
        Score::new(i16::max_value()/2)
    }

    pub fn min() -> Score {
        Score::new(-1 * i16::max_value()/2)
    }

    pub fn max_at_depth(depth: usize) -> Score {
        Score::new(Score::max().0 - depth as i16)
    }

    pub fn min_at_depth(depth: usize) -> Score {
        Score::new(Score::min().0 + depth as i16)
    }

    pub fn flipped(&self) -> Score {
        Score::new(self.0 * -1)
    }

    pub fn store_u16(&self) -> u16 {
        (self.0 as i32 - i16::min_value() as i32) as u16
    }

    pub fn unstore_u16(u: u16) -> Score {
        Score::new( (u as i32 + i16::min_value() as i32) as i16 )
    }

}

impl Phase {
    pub fn unwrap(&self) -> u16 { self.0 }

    fn recompute(board: &Board) -> Phase {
        let knight_phase = 1;
        let bishop_phase = 1;
        let rook_phase = 2;
        let queen_phase = 4;
        let total_phase = knight_phase*4 + bishop_phase*4 + rook_phase*4 + queen_phase*2;

        let mut phase = total_phase;

        use PieceType::*;
        use Color::*;

        phase -= knight_phase * board.get_pieces(White, Knight).population() as u16;
        phase -= knight_phase * board.get_pieces(Black, Knight).population() as u16;
        phase -= bishop_phase * board.get_pieces(White, Bishop).population() as u16;
        phase -= bishop_phase * board.get_pieces(Black, Bishop).population() as u16;
        phase -= rook_phase * board.get_pieces(White, Rook).population() as u16;
        phase -= rook_phase * board.get_pieces(Black, Rook).population() as u16;
        phase -= queen_phase * board.get_pieces(White, Queen).population() as u16;
        phase -= queen_phase * board.get_pieces(Black, Queen).population() as u16;

        Phase((phase * 256 + (total_phase / 2)) / total_phase)
    }
}


impl Score {
    pub fn recompute(game: &Game, search_depth: usize) -> Score {
        use PieceType::*;
        use Color::*;

        match game.outcome {
            Some(GameResult::Win(White)) => return Score::max_at_depth(search_depth),
            Some(GameResult::Win(Black)) => return Score::min_at_depth(search_depth),
            Some(GameResult::Draw) => return Score::new(0),
            None => {}
        }

        let material_score = |ptype: PieceType| {
            let diff = game.board.get_pieces(White, ptype).population() as i16
                     - game.board.get_pieces(Black, ptype).population() as i16;

            let value: i16 = material_value(ptype);
            return value * diff;
        };

        let piece_square_score = |ptype: PieceType| -> (i16, i16) {
            let mut diff = (0, 0);

            for color in [White, Black].iter() {
                for sq in game.board.get_pieces(*color, ptype) {
                    let (x,y) = piece_square_value(*color, ptype, sq);
                    diff.0 += x;
                    diff.1 += y;
                }
            }

            return diff;
        };

        let mut mat_score: i16 = 0;
        let mut psq_score: (i16,i16) = (0,0);

        for ptype in PieceType::all() {
            mat_score += material_score(*ptype);
            let (x,y) = piece_square_score(*ptype);
            psq_score.0 += x;
            psq_score.1 += y;
        }

        let phase = Phase::recompute(&game.board).unwrap() as f32;
        let midgame_score = psq_score.0 as f32 + mat_score as f32;
        let endgame_score = psq_score.1 as f32 + mat_score as f32;

        let eval = ((midgame_score * (256.0 - phase)) + (endgame_score * phase)) / 256.0;

        return Score::new(eval as i16);
    }
}

fn piece_square_value(color: Color, ptype: PieceType, sq: Square) -> (i16,i16) {
    let idx = match color {
        Color::White => 63 - sq.idx(),
        Color::Black => 63 - sq.bitrep().flip_color().bitscan_forward().idx()
    };

    let sf = match color {
        Color::White => 1,
        Color::Black => -1
    };

    let (mid_val, end_val): (i16,i16) = match ptype {
        PieceType::Pawn   => unsafe { *PAWN_TABLE.get_unchecked(idx) }
        PieceType::Knight => unsafe { *KNIGHT_TABLE.get_unchecked(idx) },
        PieceType::Bishop => unsafe { *BISHOP_TABLE.get_unchecked(idx) },
        PieceType::Rook   => unsafe { *ROOK_TABLE.get_unchecked(idx) },
        PieceType::Queen  => unsafe { *QUEEN_TABLE.get_unchecked(idx) },
        PieceType::King   => unsafe { *KING_TABLE.get_unchecked(idx) }
    };

    return (sf * mid_val, sf * end_val);
}

fn material_value(ptype: PieceType) -> i16 {
    match ptype {
        PieceType::Pawn   => 100,
        PieceType::Knight => 320,
        PieceType::Bishop => 330,
        PieceType::Rook   => 500,
        PieceType::Queen  => 900,
        PieceType::King   => 20000
    }
}

// (middle-game, end-game)
const PAWN_TABLE: [(i16,i16); 64] =
[
    ( 0 , 0 ) , ( 0 , 0 ) , ( 0  , 0 ) , ( 0  , 0 ) , ( 0  , 0 ) , ( 0  , 0 ) , ( 0 , 0 ) , ( 0 , 0 ) ,
    ( 2 , 0 ) , ( 7 , 0 ) , ( 12 , 0 ) , ( 17 , 0 ) , ( 17 , 0 ) , ( 12 , 0 ) , ( 7 , 0 ) , ( 2 , 0 ) ,
    ( 0 , 0 ) , ( 5 , 0 ) , ( 10 , 0 ) , ( 15 , 0 ) , ( 15 , 0 ) , ( 10 , 0 ) , ( 5 , 0 ) , ( 0 , 0 ) ,
    ( 0 , 0 ) , ( 5 , 0 ) , ( 10 , 0 ) , ( 25 , 0 ) , ( 25 , 0 ) , ( 10 , 0 ) , ( 5 , 0 ) , ( 0 , 0 ) ,
    ( 0 , 0 ) , ( 5 , 0 ) , ( 10 , 0 ) , ( 35 , 0 ) , ( 35 , 0 ) , ( 10 , 0 ) , ( 5 , 0 ) , ( 0 , 0 ) ,
    ( 0 , 0 ) , ( 5 , 0 ) , ( 10 , 0 ) , ( 15 , 0 ) , ( 15 , 0 ) , ( 10 , 0 ) , ( 5 , 0 ) , ( 0 , 0 ) ,
    ( 2 , 0 ) , ( 7 , 0 ) , ( 12 , 0 ) , ( -5 , 0 ) , ( -5 , 0 ) , ( 12 , 0 ) , ( 7 , 0 ) , ( 2 , 0 ) ,
    ( 0 , 0 ) , ( 0 , 0 ) , ( 0  , 0 ) , ( 0  , 0 ) , ( 0  , 0 ) , ( 0  , 0 ) , ( 0 , 0 ) , ( 0 , 0 )
];

const KNIGHT_TABLE: [(i16,i16); 64] =
[
  ( -50 , 0 ) , ( 3   , 3  ) , ( 6   , 6  ) , ( 9   , 9  ) , ( 9  , 9  ) , ( 6  , 6  ) , ( 3   , 3  ) , ( -50 , 0 ) ,
  ( 3  , 3 ) ,  ( 12  , 12 ) , ( 15  , 15 ) , ( 18  , 18 ) , ( 18 , 18 ) , ( 15 , 15 ) , ( 12  , 12 ) , ( 3   , 3 ) ,
  ( 6  , 6 ) ,  ( 15  , 15 ) , ( 21  , 21 ) , ( 27  , 24 ) , ( 27 , 24 ) , ( 21 , 21 ) , ( 15  , 15 ) , ( 6   , 6 ) ,
  ( 9  , 9 ) ,  ( 18  , 18 ) , ( 27  , 24 ) , ( 32  , 27 ) , ( 32 , 27 ) , ( 27 , 24 ) , ( 18  , 18 ) , ( 9   , 9 ) ,
  ( 9  , 9 ) ,  ( 18  , 18 ) , ( 24  , 24 ) , ( 27  , 27 ) , ( 27 , 27 ) , ( 24 , 24 ) , ( 18  , 18 ) , ( 9   , 9 ) ,
  ( 6  , 6 ) ,  ( 15  , 15 ) , ( 21  , 21 ) , ( 24  , 24 ) , ( 24 , 24 ) , ( 21 , 21 ) , ( 15  , 15 ) , ( 6   , 6 ) ,
  ( 3  , 3 ) ,  ( 12  , 12 ) , ( 15  , 15 ) , ( 18  , 18 ) , ( 18 , 18 ) , ( 15 , 15 ) , ( 12  , 12 ) , ( 3   , 3 ) ,
  ( -15 , 0 ) , ( -12 , 3  ) , ( -9 , 6  ) ,  ( -6 , 9  ) ,  ( -6 , 9  ) , ( -9 , 6  ) , ( -12 , 3  ) , ( -15 , 0 )
];

const BISHOP_TABLE: [(i16,i16); 64] =
[
  (19  , 20) , (16 , 22) , (17 , 24) , (18 , 26) , (18 , 26) , (17 , 24) , (16 , 22) , (19  , 20) ,
  (-14 , 22) , (23 , 28) , (20 , 30) , (21 , 32) , (21 , 32) , (20 , 30) , (23 , 28) , (-14 , 22) ,
  (17  , 24) , (20 , 30) , (26 , 34) , (23 , 36) , (23 , 36) , (26 , 34) , (20 , 30) , (17  , 24) ,
  (18  , 26) , (21 , 32) , (23 , 36) , (28 , 38) , (28 , 38) , (23 , 36) , (21 , 32) , (18  , 26) ,
  (18  , 26) , (21 , 32) , (23 , 36) , (28 , 38) , (28 , 38) , (23 , 36) , (21 , 32) , (18  , 26) ,
  (17  , 24) , (20 , 30) , (26 , 34) , (23 , 36) , (23 , 36) , (26 , 34) , (20 , 30) , (17  , 24) ,
  (16  , 22) , (23 , 28) , (20 , 30) , (21 , 32) , (21 , 32) , (20 , 30) , (23 , 28) , (16  , 22) ,
  (9   , 20) , (6  , 22) , (7  , 24) , (8  , 26) , (8  , 26) , (7  , 24) , (6  , 22) , (9   , 20)
];

const ROOK_TABLE: [(i16,i16); 64] =
[
   (0 , 25),  (3 , 25),  (6 , 25),  (9 , 25),  (9 , 25),  (6 , 25),  (3 , 25),  (0 , 25),
   (25, 25),  (28, 25),  (31, 25),  (34, 25),  (34, 25),  (31, 25),  (28, 25),  (25 , 25),
   (0 , 25),  (3 , 25),  (6 , 25),  (9 , 25),  (9 , 25),  (6 , 25),  (3 , 25),  (0 , 25),
   (0 , 25),  (3 , 25),  (6 , 25),  (9 , 25),  (9 , 25),  (6 , 25),  (3 , 25),  (0 , 25),
   (0 , 25),  (3 , 25),  (6 , 25),  (9 , 25),  (9 , 25),  (6 , 25),  (3 , 25),  (0 , 25),
   (0 , 25),  (3 , 25),  (6 , 25),  (9 , 25),  (9 , 25),  (6 , 25),  (3 , 25),  (0 , 25),
   (0 , 25),  (3 , 25),  (6 , 25),  (9 , 25),  (9 , 25),  (6 , 25),  (3 , 25),  (0 , 25),
   (1 , 25),  (4 , 25),  (7 , 25),  (10, 25),  (10, 25),  (7 , 25),  (4 , 25),  (1 , 25)
];

const QUEEN_TABLE: [(i16,i16); 64] =
[
  (-20,30),(-10,30),(-10,30),( -5,30 ),(-5,30),(-10,30),(-10,30),(-20,30),
  (-10,30),(  0,30),(  0,30),(  0,30 ),( 0,30),(  0,30),(  0,30),(-10,30),
  (-10,30),(  0,30),(  5,30),(  5,30 ),( 5,30),(  5,30),(  0,30),(-10,30),
  ( -5,30),(  0,30),(  5,30),(  5,30 ),( 5,30),(  5,30),(  0,30),( -5,30),
  (  0,30),(  0,30),(  5,30),(  5,30 ),( 5,30),(  5,30),(  0,30),( -5,30),
  (-10,30),(  5,30),(  5,30),(  5,30 ),( 5,30),(  5,30),(  0,30),(-10,30),
  (-10,30),(  0,30),(  5,30),(  0,30 ),( 0,30),(  0,30),(  0,30),(-10,30),
  (-20,30),(-10,30),(-10,30),( -5,30 ),(-5,30),(-10,30),(-10,30),(-20, 30)
];

const KING_TABLE: [(i16,i16); 64] =
[
 ( -175 , 0  ) , ( -175 , 10 ) , ( -175 , 20 ) , ( -175 , 30 ) , ( -175 , 30 ) , ( -175 , 20 ) , ( -175 , 10 ) , ( -175 , 0  ) ,
 ( -150 , 10 ) , ( -150 , 40 ) , ( -150 , 50 ) , ( -150 , 60 ) , ( -150 , 60 ) , ( -150 , 50 ) , ( -150 , 40 ) , ( -150 , 10 ) ,
 ( -125 , 20 ) , ( -125 , 50 ) , ( -125 , 70 ) , ( -125 , 80 ) , ( -125 , 80 ) , ( -125 , 70 ) , ( -125 , 50 ) , ( -125 , 20 ) ,
 ( -100 , 30 ) , ( -100 , 60 ) , ( -100 , 80 ) , ( -100 , 90 ) , ( -100 , 90 ) , ( -100 , 80 ) , ( -100 , 60 ) , ( -100 , 30 ) ,
 ( -75  , 30 ) , ( -75  , 60 ) , ( -75  , 80 ) , ( -75  , 90 ) , ( -75  , 90 ) , ( -75  , 80 ) , ( -75  , 60 ) , ( -75  , 30 ) ,
 ( -50  , 20 ) , ( -50  , 50 ) , ( -50  , 70 ) , ( -50  , 80 ) , ( -50  , 80 ) , ( -50  , 70 ) , ( -50  , 50 ) , ( -50  , 20 ) ,
 ( 50   , 10 ) , ( 50   , 40 ) , ( 0   , 50 ) ,  ( 0   , 60 ) ,  ( 0   , 60 ) ,  ( 0   , 50 ) ,  ( 50   , 40 ) , ( 50   , 10 ) ,
 ( 50   , 0  ) , ( 50   , 10 ) , ( 0   , 20 ) ,  ( 0   , 30 ) ,  ( 20   , 30 ) , ( 0   , 20 ) ,  ( 50   , 10 ) , ( 50   , 0  )
];

#[cfg(test)]
mod test {
    use eval::*;

    #[test]
    fn flip() {
        for _ in 0 .. 100000 {
            let original_game = Game::random_game();
            let mut flipped_game = original_game;
            flipped_game.flip_color();
            let original_score = Score::recompute(&original_game, 0);
            let flipped_score = Score::recompute(&flipped_game, 0);
            if original_score != flipped_score.flipped() {
                original_game.board.print();
                assert!(false, format!("{:?} {:?}", original_score, flipped_score));
            }
        }
    }
}
