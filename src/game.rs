use bitboard::*;
use board::*;
use core::*;
use moves::*;
use moves::*;
use tables::*;

use std::collections::HashMap;
use std::num;

bitflags! {
    pub struct CastlingRights: u8 {
        const WHITE_KINGSIDE  = 0b0001;
        const WHITE_QUEENSIDE = 0b0010;
        const BLACK_KINGSIDE  = 0b0100;
        const BLACK_QUEENSIDE = 0b1000;
    }
}

pub enum GameResult {
    Win(Color),
    Draw
}

#[derive(Clone, Copy)]
pub struct Game {
    pub board: Board,
    pub to_move: Color,
    pub ep_square: Option<Square>,
    pub castling_rights: CastlingRights,
    pub fifty_move_count: u8,
    pub moves_played: u16,
    pub king_attackers: Bitboard
}

impl Game {
    pub fn starting_position() -> Game {
        Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn empty_position() -> Game {
        Game {
            board: Board::empty_position(),
            to_move: Color::White,
            ep_square: None,
            castling_rights: CastlingRights::empty(),
            fifty_move_count: 0,
            moves_played: 0,
            king_attackers: Bitboard::new(0)
        }
    }

    pub fn outcome(&self, move_count: usize) -> Option<GameResult> {
        let check_multiplicity  = self.king_attackers.population();

        if move_count == 0 && check_multiplicity > 0 {
            return Some(GameResult::Win(!self.to_move));
        }

        if move_count == 0 && check_multiplicity == 0 {
            return Some(GameResult::Draw);
        }

        if self.fifty_move_count >= 50 {
            return Some(GameResult::Draw);
        }

        return None;
    }

    pub fn to_fen(&self) -> String {
        use PieceType::*;
        use Color::*;

        let mut board_str = String::new();
        let mut empty_tally = 0;

        for idx in (0..64).rev() {
            let sq = Square::new(idx);
            let wrapped_across_row = sq.unwrap() % 8 == 7;

            let maybe_piece = self.board.piece_at(sq);

            if (maybe_piece.is_some() || wrapped_across_row) && empty_tally > 0 {
                assert!(empty_tally <= 8);
                board_str.push_str(&empty_tally.to_string());
                empty_tally = 0;
            }

            if wrapped_across_row && idx < 63 {
                board_str.push('/');
            }

            match maybe_piece {
                Some(piece) => {
                    match (piece.color, piece.ptype) {
                        (Black , Pawn  ) => board_str.push('p'),
                        (Black , Knight) => board_str.push('n'),
                        (Black , Bishop) => board_str.push('b'),
                        (Black , Rook  ) => board_str.push('r'),
                        (Black , Queen ) => board_str.push('q'),
                        (Black , King  ) => board_str.push('k'),
                        (White , Pawn  ) => board_str.push('P'),
                        (White , Knight) => board_str.push('N'),
                        (White , Bishop) => board_str.push('B'),
                        (White , Rook  ) => board_str.push('R'),
                        (White , Queen ) => board_str.push('Q'),
                        (White , King  ) => board_str.push('K'),
                    }
                }
                None => empty_tally += 1
            }
        }

        if empty_tally > 0 {
            board_str.push_str(&empty_tally.to_string());
        }

        let to_move_str = match self.to_move {
            White => "w".to_string(),
            Black => "b".to_string()
        };

        let mut castling_str = String::new();

        if (self.castling_rights == CastlingRights::empty()) {
            castling_str = "-".to_string();
        } else {
            if self.castling_rights.intersects(CastlingRights::WHITE_KINGSIDE) {
                castling_str.push('K');
            }
            if self.castling_rights.intersects(CastlingRights::WHITE_QUEENSIDE) {
                castling_str.push('Q');
            }
            if self.castling_rights.intersects(CastlingRights::BLACK_KINGSIDE) {
                castling_str.push('k');
            }
            if self.castling_rights.intersects(CastlingRights::BLACK_QUEENSIDE) {
                castling_str.push('q');
            }
        }

        let ep_square_str = match self.ep_square {
            Some(sq) => sq.to_algebraic().to_string(),
            None => "-".to_string()
        };

        return [board_str,
                to_move_str,
                castling_str,
                ep_square_str,
                self.fifty_move_count.to_string(),
                self.moves_played.to_string()
        ].join(" ");
    }

    pub fn from_fen(fen: &'static str) -> Option<Game> {
        let words: Vec<&str> = fen.split(' ').collect();

        if words.len() != 6 {
            return None;
        }

        let mut game = Game::empty_position();

        use PieceType::*;
        use Color::*;

        { // build up the game board
            let mut current_square: Square = Square::new(63);

            let decrement_square = |sq: &mut Square, n: u32| {
                if sq.unwrap() > 0 {
                    *sq = Square::new(sq.unwrap() - n);
                }
            };

            let mut add_piece = |color: Color, piece: PieceType, sq: &mut Square| {
                game.board.set_piece_bit(color, piece, *sq);
                decrement_square(sq, 1);
            };

            for ch in words[0].chars() {
                match ch {
                    'p' => add_piece(Black , Pawn   , &mut current_square) ,
                    'n' => add_piece(Black , Knight , &mut current_square) ,
                    'b' => add_piece(Black , Bishop , &mut current_square) ,
                    'r' => add_piece(Black , Rook   , &mut current_square) ,
                    'q' => add_piece(Black , Queen  , &mut current_square) ,
                    'k' => add_piece(Black , King   , &mut current_square) ,
                    'P' => add_piece(White , Pawn   , &mut current_square) ,
                    'N' => add_piece(White , Knight , &mut current_square) ,
                    'B' => add_piece(White , Bishop , &mut current_square) ,
                    'R' => add_piece(White , Rook   , &mut current_square) ,
                    'Q' => add_piece(White , Queen  , &mut current_square) ,
                    'K' => add_piece(White , King   , &mut current_square) ,
                    '1' => decrement_square(&mut current_square, 1),
                    '2' => decrement_square(&mut current_square, 2),
                    '3' => decrement_square(&mut current_square, 3),
                    '4' => decrement_square(&mut current_square, 4),
                    '5' => decrement_square(&mut current_square, 5),
                    '6' => decrement_square(&mut current_square, 6),
                    '7' => decrement_square(&mut current_square, 7),
                    '8' =>
                        if current_square.idx() == 7 {
                            decrement_square(&mut current_square, 7)
                        } else {
                            decrement_square(&mut current_square, 8)
                        },
                    '/' => {},
                    _ => return None
                }
            }
        }

        match words[1] {
            "w" => game.to_move = White,
            "b" => game.to_move = Black,
            _ => return None
        }

        for ch in words[2].chars() {
            match ch {
                'K' => game.castling_rights |= CastlingRights::WHITE_KINGSIDE,
                'Q' => game.castling_rights |= CastlingRights::WHITE_QUEENSIDE,
                'k' => game.castling_rights |= CastlingRights::BLACK_KINGSIDE,
                'q' => game.castling_rights |= CastlingRights::BLACK_QUEENSIDE,
                '-' => {},
                _ => return None
            }
        }

        match words[3] {
            "-" => game.ep_square = None,
            _ => match Square::from_algebraic(words[3]) {
                None => return None,
                Some(sq) => game.ep_square = Some(sq)
            }
        }

        match words[4].parse::<u8>() {
            Err(_) => return None,
            Ok(x) => game.fifty_move_count = x
        }

        match words[5].parse::<u16>() {
            Err(_) => return None,
            Ok(x) => game.moves_played = x
        }

        let king_square     = game.board.get_king_square(game.to_move);
        game.king_attackers = game.board.attackers(king_square, !game.to_move);

        return Some(game);
    }

    pub fn make_move(&mut self, m: Move) {
        let from_sq        = m.from();
        let from_bit       = from_sq.bitrep();
        let to_sq          = m.to();
        let to_bit         = to_sq.bitrep();
        let from_to_bit    = from_bit | to_bit;
        let is_capture     = m.is_capture();
        let is_promotion   = m.is_promotion();
        let flag           = m.flag();
        let moving_color   = self.to_move;
        let opponent_color = !moving_color;

        let king_square = self.board.get_king_square(opponent_color);

        use Color::*;
        use PieceType::*;

        let moved_piece = self.board.piece_at(from_sq).unwrap().ptype;
        let mut captured_piece = None;

        if (is_capture) {
            match to_sq.idx() {
                0 => self.castling_rights.remove(CastlingRights::WHITE_KINGSIDE),
                7 => self.castling_rights.remove(CastlingRights::WHITE_QUEENSIDE),
                56 => self.castling_rights.remove(CastlingRights::BLACK_KINGSIDE),
                63 => self.castling_rights.remove(CastlingRights::BLACK_QUEENSIDE),
                _ => {}
            }

            captured_piece =
                if flag == EP_CAPTURE_FLAG {
                    match opponent_color {
                        White => self.board.piece_at(Square::new(self.ep_square.unwrap().unwrap() + 8)),
                        Black => self.board.piece_at(Square::new(self.ep_square.unwrap().unwrap() - 8))
                    }
                } else {
                    self.board.piece_at(to_sq)
                };
        }

        assert!(is_capture == captured_piece.is_some());

        //TODO: add moving/captured piece type to Move structure
        match moved_piece {
            Pawn => {
                *self.board.get_pieces_mut(self.to_move, Pawn) ^= from_to_bit;
                *self.board.occupied_by_mut(self.to_move) ^= from_to_bit;

                if flag == DOUBLE_PAWN_PUSH_FLAG {
                    self.ep_square = match moving_color {
                        White => Some(Square::new(to_sq.unwrap() - 8)),
                        Black => Some(Square::new(to_sq.unwrap() + 8))
                    }
                }

                // TODO: promotions
                if is_capture {
                    if flag == EP_CAPTURE_FLAG {
                        assert!(self.ep_square.is_some());

                        let captured_bit = match moving_color {
                            White => self.ep_square.unwrap().bitrep().shifted_down(),
                            Black => self.ep_square.unwrap().bitrep().shifted_up()
                        };

                        *self.board.get_pieces_mut(opponent_color, Pawn) ^= captured_bit;
                        *self.board.occupied_by_mut(opponent_color) ^= captured_bit;
                    } else {
                        *self.board.get_pieces_mut(opponent_color, captured_piece.unwrap().ptype) ^= to_bit;
                        *self.board.occupied_by_mut(opponent_color) ^= to_bit;
                    }
                }

                if is_promotion {
                    *self.board.get_pieces_mut(moving_color, Pawn) &= !to_bit;

                    if flag == KNIGHT_PROMO_FLAG || flag == KNIGHT_PROMO_CAPTURE_FLAG {
                        *self.board.get_pieces_mut(moving_color, Knight) |= to_bit;
                    } else if flag == BISHOP_PROMO_FLAG || flag == BISHOP_PROMO_CAPTURE_FLAG {
                        *self.board.get_pieces_mut(moving_color, Bishop) |= to_bit;
                    } else if flag == ROOK_PROMO_FLAG || flag == ROOK_PROMO_CAPTURE_FLAG {
                        *self.board.get_pieces_mut(moving_color, Rook) |= to_bit;
                    } else if flag == QUEEN_PROMO_FLAG || flag == QUEEN_PROMO_CAPTURE_FLAG {
                        *self.board.get_pieces_mut(moving_color, Queen) |= to_bit;
                    }

                }

            },

            Knight => {
                *self.board.get_pieces_mut(self.to_move, Knight) ^= from_to_bit;
                *self.board.occupied_by_mut(self.to_move) ^= from_to_bit;
                if is_capture {
                    *self.board.get_pieces_mut(opponent_color, captured_piece.unwrap().ptype) ^= to_bit;
                    *self.board.occupied_by_mut(opponent_color) ^= to_bit;
                }
            },

            Bishop => {
                *self.board.get_pieces_mut(self.to_move, Bishop) ^= from_to_bit;
                *self.board.occupied_by_mut(self.to_move) ^= from_to_bit;
                if is_capture {
                    *self.board.get_pieces_mut(opponent_color, captured_piece.unwrap().ptype) ^= to_bit;
                    *self.board.occupied_by_mut(opponent_color) ^= to_bit;
                }
            },

            Rook => {
                *self.board.get_pieces_mut(self.to_move, Rook) ^= from_to_bit;
                *self.board.occupied_by_mut(self.to_move) ^= from_to_bit;

                match moving_color {
                    White =>
                        if from_sq.idx() == 0 {
                            self.castling_rights.remove(CastlingRights::WHITE_KINGSIDE);
                        } else if from_sq.idx() == 7 {
                            self.castling_rights.remove(CastlingRights::WHITE_QUEENSIDE);
                        },

                    Black =>
                        if from_sq.idx() == 63 {
                            self.castling_rights.remove(CastlingRights::BLACK_QUEENSIDE);
                        } else if from_sq.idx() == 56 {
                            self.castling_rights.remove(CastlingRights::BLACK_KINGSIDE);
                        }
                }

                if is_capture {
                    *self.board.get_pieces_mut(opponent_color, captured_piece.unwrap().ptype) ^= to_bit;
                    *self.board.occupied_by_mut(opponent_color) ^= to_bit;
                }
            },

            Queen => {
                *self.board.get_pieces_mut(self.to_move, Queen) ^= from_to_bit;
                *self.board.occupied_by_mut(self.to_move) ^= from_to_bit;
                if is_capture {
                    *self.board.get_pieces_mut(opponent_color, captured_piece.unwrap().ptype) ^= to_bit;
                    *self.board.occupied_by_mut(opponent_color) ^= to_bit;
                }
            },

            King => {
                *self.board.get_pieces_mut(self.to_move, King) ^= from_to_bit;
                *self.board.occupied_by_mut(self.to_move) ^= from_to_bit;

                match moving_color {
                    White => {
                        self.castling_rights.remove(CastlingRights::WHITE_KINGSIDE);
                        self.castling_rights.remove(CastlingRights::WHITE_QUEENSIDE);
                        if (flag == KING_CASTLE_FLAG) {
                            let rook_bit = Square::new(0).bitrep() | Square::new(2).bitrep();
                            *self.board.get_pieces_mut(self.to_move, Rook) ^= rook_bit;
                            *self.board.occupied_by_mut(self.to_move) ^= rook_bit;
                        }
                        if (flag == QUEEN_CASTLE_FLAG) {
                            let rook_bit = Square::new(7).bitrep() | Square::new(4).bitrep();
                            *self.board.get_pieces_mut(self.to_move, Rook) ^= rook_bit;
                            *self.board.occupied_by_mut(self.to_move) ^= rook_bit;
                        }
                    }

                    Black => {
                        self.castling_rights.remove(CastlingRights::BLACK_QUEENSIDE);
                        self.castling_rights.remove(CastlingRights::BLACK_KINGSIDE);
                        if (flag == KING_CASTLE_FLAG) {
                            let rook_bit = Square::new(56).bitrep() | Square::new(58).bitrep();
                            *self.board.get_pieces_mut(self.to_move, Rook) ^= rook_bit;
                            *self.board.occupied_by_mut(self.to_move) ^= rook_bit;
                        }
                        if (flag == QUEEN_CASTLE_FLAG) {
                            let rook_bit = Square::new(63).bitrep() | Square::new(60).bitrep();
                            *self.board.get_pieces_mut(self.to_move, Rook) ^= rook_bit;
                            *self.board.occupied_by_mut(self.to_move) ^= rook_bit;
                        }
                    }
                }

                if is_capture {
                    *self.board.get_pieces_mut(opponent_color, captured_piece.unwrap().ptype) ^= to_bit;
                    *self.board.occupied_by_mut(opponent_color) ^= to_bit;
                }
            },
        }

        if flag != DOUBLE_PAWN_PUSH_FLAG {
            self.ep_square = None;
        }

        if is_capture || moved_piece == Pawn {
            self.fifty_move_count = 0;
        } else {
            // self.fifty_move_count += 1;
        }

        self.to_move = !self.to_move;
    }
}

#[cfg(test)]
mod test {
    use game::*;
    #[test]
    fn test_fen() {
        let fen_strings: Vec<&'static str> = vec![
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            "r1bqkbnr/pp1npp1p/2pp2p1/8/2PPP3/2N1B3/PP3PPP/R2QKBNR b KQkq - 1 5",
            "r2q1rk1/1p1nbppp/pn1pb3/4p3/4P1PP/1NN1BP2/PPPQ4/1K1R1B1R b - - 0 13",
            "r2qnrk1/4bppp/1B1pb3/p3p1P1/1p2PP2/1N6/PPPQN2P/1K1R1B1R b - - 0 16",
            "r1bq1rk1/ppp3bp/n2p2p1/3PpP1n/2P5/2N2NP1/PP2BP1P/R1BQ1RK1 b - - 0 10",
            "5r2/4q1pk/2bp1p1p/1p2n3/3QPB2/1B1P3P/1PP3P1/r4RK1 w - - 0 25"

        ];

        for fen in fen_strings.iter() {
            let g = Game::from_fen(fen).unwrap();
            assert!(&g.to_fen() == fen);
        }
    }
}
