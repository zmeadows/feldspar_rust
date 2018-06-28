use bitboard::*;
use board::*;
use core::*;
use moves::*;
use tables::*;
use eval::*;
use movegen::*;
use zobrist::*;

use std::str::SplitWhitespace;
use rand::{thread_rng, Rng};

#[derive(Debug,PartialEq,Clone, Copy)]
pub enum GameResult {
    Win(Color),
    Draw
}

#[derive(PartialEq, Clone, Copy)]
pub struct Game {
    pub board: Board,
    pub to_move: Color,
    pub ep_square: Option<Square>,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u8,
    pub fullmoves: u16,
    pub king_attackers: Bitboard,
    pub outcome: Option<GameResult>,
    pub hash: Hash
}

impl Game {
    #[allow(dead_code)]
    pub fn starting_position() -> Game {
        Game::from_fen_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn empty_position() -> Game {
        Game {
            board: Board::empty_position(),
            to_move: Color::White,
            ep_square: None,
            castling_rights: CastlingRights::empty(),
            halfmove_clock: 0,
            fullmoves: 1,
            king_attackers: Bitboard::none_set(),
            outcome: None,
            hash: Hash::empty()
        }
    }

    pub fn in_check(&self) -> bool {
        self.king_attackers.population() > 0
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
                debug_assert!(empty_tally <= 8);
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

        if self.castling_rights == CastlingRights::empty() {
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
                self.halfmove_clock.to_string(),
                self.fullmoves.to_string()
        ].join(" ");
    }

    pub fn from_fen_str<'a>(fen: &'a str) -> Option<Game> {
        let mut fen_split = fen.split_whitespace();
        Game::from_fen(&mut fen_split)
    }

    pub fn from_fen<'a>(args: &mut SplitWhitespace<'a>) -> Option<Game> {
        let mut game = Game::empty_position();

        use PieceType::*;
        use Color::*;

        { // build up the game board
            let mut current_square: Square = Square::new(63);

            let decrement_square = |sq: &mut Square, n: u32| {
                if sq.unwrap() >= n {
                    *sq = Square::new(sq.unwrap() - n);
                } else {
                    *sq = Square::new(0);
                }
            };

            let mut add_piece = |piece_color: Color, piece_type: PieceType, sq: &mut Square| {
                game.board.set_piece_bit(piece_color, piece_type, *sq);
                decrement_square(sq, 1);
            };

            for ch in args.next().expect("Missing FEN string").chars() {
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
                    '8' => decrement_square(&mut current_square, 8),
                    '/' => {},
                    _ => return None
                }
            }
        }

        match args.next().expect("Missing color-to-move in FEN string") {
            "w" => game.to_move = White,
            "b" => game.to_move = Black,
            _ => return None
        }

        for ch in args.next().expect("Missing castling rights in FEN string").chars() {
            match ch {
                'K' => game.castling_rights |= CastlingRights::WHITE_KINGSIDE,
                'Q' => game.castling_rights |= CastlingRights::WHITE_QUEENSIDE,
                'k' => game.castling_rights |= CastlingRights::BLACK_KINGSIDE,
                'q' => game.castling_rights |= CastlingRights::BLACK_QUEENSIDE,
                '-' => {},
                _ => return None
            }
        }

        match Square::from_algebraic(args.next().expect("Missing en-passante square in FEN string")) {
            None => game.ep_square = None,
            Some(sq) => game.ep_square = Some(sq)
        }

        match args.next().expect("Missing fifty move count in FEN string").parse::<u8>() {
            Err(_) => return None,
            Ok(x) => game.halfmove_clock = x
        }

        match args.next().expect("Missing move count in FEN string").parse::<u16>() {
            Err(_) => return None,
            Ok(x) => game.fullmoves = x
        }

        let king_square     = game.board.get_king_square(game.to_move);
        game.king_attackers = game.board.attackers(king_square, !game.to_move);

        game.hash = Hash::new(&game);

        return Some(game);
    }

    pub fn make_move(&mut self, m: Move) {
        use Color::*;
        use PieceType::*;

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
        let moved_ptype    = m.moved_piece();
        let moved_piece    = Piece::new(moving_color, moved_ptype);
        let captured_ptype = m.captured_piece();

        self.hash.change_piece(moving_color, moved_ptype, from_sq);
        self.hash.change_piece(moving_color, moved_ptype, to_sq);

        *self.board.get_pieces_mut(self.to_move, moved_ptype) ^= from_to_bit;
        *self.board.occupied_by_mut(self.to_move) ^= from_to_bit;

        if is_capture {
            match to_sq.idx() {
                0 => {
                    self.hash.update_castling_rights(self.castling_rights);
                    self.castling_rights.remove(CastlingRights::WHITE_KINGSIDE);
                    self.hash.update_castling_rights(self.castling_rights);
                }
                7 => {
                    self.hash.update_castling_rights(self.castling_rights);
                    self.castling_rights.remove(CastlingRights::WHITE_QUEENSIDE);
                    self.hash.update_castling_rights(self.castling_rights);
                }
                56 => {
                    self.hash.update_castling_rights(self.castling_rights);
                    self.castling_rights.remove(CastlingRights::BLACK_KINGSIDE);
                    self.hash.update_castling_rights(self.castling_rights);
                }
                63 => {
                    self.hash.update_castling_rights(self.castling_rights);
                    self.castling_rights.remove(CastlingRights::BLACK_QUEENSIDE);
                    self.hash.update_castling_rights(self.castling_rights);
                }
                _ => {}
            }


            if moved_ptype != Pawn {
                *self.board.get_pieces_mut(opponent_color, captured_ptype.unwrap()) ^= to_bit;
                *self.board.occupied_by_mut(opponent_color) ^= to_bit;
                self.hash.change_piece(opponent_color, captured_ptype.unwrap(), to_sq);
            }
        }

        match moved_ptype {
            Pawn => {

                if flag == DOUBLE_PAWN_PUSH_FLAG {
                    if (self.ep_square.is_some()) {
                        self.hash.modify_ep_square(self.ep_square.unwrap());
                    }

                    self.ep_square = match moving_color {
                        White => Some(Square::new(to_sq.unwrap() - 8)),
                        Black => Some(Square::new(to_sq.unwrap() + 8))
                    };

                    self.hash.modify_ep_square(self.ep_square.unwrap());
                }

                if is_capture {
                    if flag == EP_CAPTURE_FLAG {
                        debug_assert!(self.ep_square.is_some());

                        let captured_bit = match moving_color {
                            White => self.ep_square.unwrap().bitrep().shifted_down(),
                            Black => self.ep_square.unwrap().bitrep().shifted_up()
                        };

                        let captured_sq = captured_bit.bitscan_forward();

                        *self.board.get_pieces_mut(opponent_color, Pawn) ^= captured_bit;
                        *self.board.occupied_by_mut(opponent_color) ^= captured_bit;
                        self.hash.change_piece(opponent_color, captured_ptype.unwrap(), captured_sq);
                    } else {
                        *self.board.get_pieces_mut(opponent_color, captured_ptype.unwrap()) ^= to_bit;
                        *self.board.occupied_by_mut(opponent_color) ^= to_bit;
                        self.hash.change_piece(opponent_color, captured_ptype.unwrap(), to_sq);
                    }
                }

                if is_promotion {
                    *self.board.get_pieces_mut(moving_color, Pawn) &= !to_bit;
                    self.hash.change_piece(moving_color, Pawn, to_sq);

                    if flag == KNIGHT_PROMO_FLAG || flag == KNIGHT_PROMO_CAPTURE_FLAG {
                        *self.board.get_pieces_mut(moving_color, Knight) |= to_bit;
                        self.hash.change_piece(moving_color, Knight, to_sq);

                    } else if flag == BISHOP_PROMO_FLAG || flag == BISHOP_PROMO_CAPTURE_FLAG {
                        *self.board.get_pieces_mut(moving_color, Bishop) |= to_bit;
                        self.hash.change_piece(moving_color, Bishop, to_sq);

                    } else if flag == ROOK_PROMO_FLAG || flag == ROOK_PROMO_CAPTURE_FLAG {
                        *self.board.get_pieces_mut(moving_color, Rook) |= to_bit;
                        self.hash.change_piece(moving_color, Rook, to_sq);

                    } else if flag == QUEEN_PROMO_FLAG || flag == QUEEN_PROMO_CAPTURE_FLAG {
                        *self.board.get_pieces_mut(moving_color, Queen) |= to_bit;
                        self.hash.change_piece(moving_color, Queen, to_sq);
                    }
                }

            },

            Rook => {
                match moving_color {
                    White =>
                        if from_sq.idx() == 0 {
                            self.hash.update_castling_rights(self.castling_rights);
                            self.castling_rights.remove(CastlingRights::WHITE_KINGSIDE);
                            self.hash.update_castling_rights(self.castling_rights);
                        } else if from_sq.idx() == 7 {
                            self.hash.update_castling_rights(self.castling_rights);
                            self.castling_rights.remove(CastlingRights::WHITE_QUEENSIDE);
                            self.hash.update_castling_rights(self.castling_rights);
                        },

                    Black =>
                        if from_sq.idx() == 63 {
                            self.hash.update_castling_rights(self.castling_rights);
                            self.castling_rights.remove(CastlingRights::BLACK_QUEENSIDE);
                            self.hash.update_castling_rights(self.castling_rights);
                        } else if from_sq.idx() == 56 {
                            self.hash.update_castling_rights(self.castling_rights);
                            self.castling_rights.remove(CastlingRights::BLACK_KINGSIDE);
                            self.hash.update_castling_rights(self.castling_rights);
                        }
                }
            },

            King => {
                match moving_color {
                    White => {
                        if flag == KING_CASTLE_FLAG {
                            let rook_old_sq = Square::new(0);
                            let rook_new_sq = Square::new(2);
                            let rook_bit = rook_old_sq.bitrep() | rook_new_sq.bitrep();

                            *self.board.get_pieces_mut(self.to_move, Rook) ^= rook_bit;
                            *self.board.occupied_by_mut(self.to_move) ^= rook_bit;

                            self.hash.change_piece(moving_color, Rook, rook_old_sq);
                            self.hash.change_piece(moving_color, Rook, rook_new_sq);

                        } else if flag == QUEEN_CASTLE_FLAG {
                            let rook_old_sq = Square::new(7);
                            let rook_new_sq = Square::new(4);
                            let rook_bit = rook_old_sq.bitrep() | rook_new_sq.bitrep();

                            *self.board.get_pieces_mut(self.to_move, Rook) ^= rook_bit;
                            *self.board.occupied_by_mut(self.to_move) ^= rook_bit;

                            self.hash.change_piece(moving_color, Rook, rook_old_sq);
                            self.hash.change_piece(moving_color, Rook, rook_new_sq);
                        }

                        self.hash.update_castling_rights(self.castling_rights);
                        self.castling_rights.remove(CastlingRights::WHITE_KINGSIDE | CastlingRights::WHITE_QUEENSIDE);
                        self.hash.update_castling_rights(self.castling_rights);
                    }

                    Black => {
                        if flag == KING_CASTLE_FLAG {
                            let rook_old_sq = Square::new(56);
                            let rook_new_sq = Square::new(58);
                            let rook_bit = rook_old_sq.bitrep() | rook_new_sq.bitrep();

                            *self.board.get_pieces_mut(self.to_move, Rook) ^= rook_bit;
                            *self.board.occupied_by_mut(self.to_move) ^= rook_bit;

                            self.hash.change_piece(moving_color, Rook, rook_old_sq);
                            self.hash.change_piece(moving_color, Rook, rook_new_sq);

                        } else if flag == QUEEN_CASTLE_FLAG {
                            let rook_old_sq = Square::new(63);
                            let rook_new_sq = Square::new(60);
                            let rook_bit = rook_old_sq.bitrep() | rook_new_sq.bitrep();

                            *self.board.get_pieces_mut(self.to_move, Rook) ^= rook_bit;
                            *self.board.occupied_by_mut(self.to_move) ^= rook_bit;

                            self.hash.change_piece(moving_color, Rook, rook_old_sq);
                            self.hash.change_piece(moving_color, Rook, rook_new_sq);
                        }

                        self.hash.update_castling_rights(self.castling_rights);
                        self.castling_rights.remove(CastlingRights::BLACK_KINGSIDE | CastlingRights::BLACK_QUEENSIDE);
                        self.hash.update_castling_rights(self.castling_rights);
                    }
                }

            },

            _ => {}
        }

        if flag != DOUBLE_PAWN_PUSH_FLAG {
            if (self.ep_square.is_some()) {
                self.hash.modify_ep_square(self.ep_square.unwrap());
                self.ep_square = None;
            }
        }

        if is_capture || moved_ptype == Pawn {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        if self.to_move == Black {
            self.fullmoves += 1;
        }

        self.to_move = !self.to_move;
        self.hash.update_black_to_move();

        let opp_king_square = self.board.get_king_square(opponent_color);
        self.king_attackers = self.board.attackers(opp_king_square, !self.to_move);

        let can_move = can_move(self);

        // no moves available, game is over
        if !can_move {
            match self.king_attackers.population() {
                0 => self.outcome = Some(GameResult::Draw),
                _ => match self.to_move {
                         Color::White => self.outcome = Some(GameResult::Win(Color::Black)),
                         Color::Black => self.outcome = Some(GameResult::Win(Color::White))
                     }
            }
        }

        //NOTE: only the three-fold repetition rule isn't account for here.
    }

    pub fn random_game() -> Game {
        let mut g = Game::starting_position();

        for _ in 0 .. thread_rng().gen_range(0,80) {
            match g.outcome {
                Some(_) => break,
                None => {}
            }
            let next_moves = next_moves_standalone(&g);
            let num_moves = next_moves.len();
            if num_moves == 1 {
                g.make_move(next_moves.at(0));
            } else {
                let n = thread_rng().gen_range(0, num_moves - 1);
                g.make_move(next_moves.at(n));
            }
        }

        return g;
    }

    pub fn flip_color(&mut self) {
        use PieceType::*;
        use Color::*;

        self.to_move = !self.to_move;
        self.board.flip_color();
        self.castling_rights = self.castling_rights.flip_color();

        self.outcome = match self.outcome {
            Some(GameResult::Win(color)) => Some(GameResult::Win(!color)),
            Some(GameResult::Draw) => Some(GameResult::Draw),
            None => None
        };

        if self.ep_square.is_some() {
            let old_ep_square = self.ep_square.unwrap();
            if self.to_move == White {
                self.ep_square = Some(Square::new(old_ep_square.unwrap() - 3));
            } else {
                self.ep_square = Some(Square::new(old_ep_square.unwrap() + 3));
            }
        }

        self.king_attackers = self.king_attackers.flip_color();

        self.hash = Hash::new(self);
    }
}

#[cfg(test)]
mod test {
    use game::*;

    #[test]
    fn fen() {
        //TODO: generate random games
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
            let g = Game::from_fen_str(fen).unwrap();
            assert!(&g.to_fen() == fen);
        }
    }

    #[test]
    fn flip() {
        for _ in 0 .. 100000 {
            let mut flipped_game = Game::random_game();
            let original_game = flipped_game;
            flipped_game.flip_color();
            flipped_game.flip_color();
            assert!(flipped_game == original_game);
        }
    }
}


