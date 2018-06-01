use core::*;
use bitboard::*;
use moves::*;
use board::*;
use tables::*;

use std::collections::HashMap;

bitflags! {
    pub struct CastlingRights: u32 {
        const WHITE_KINGSIDE  = 0b0001;
        const WHITE_QUEENSIDE = 0b0010;
        const BLACK_KINGSIDE  = 0b0100;
        const BLACK_QUEENSIDE = 0b1000;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Game {
    pub board: Board,
    pub to_move: Color,
    pub ep_square: Option<Square>,
    pub castling_rights: CastlingRights,
    pub fifty_move_count: u32,
    pub moves: u32
}


impl Game {
    pub fn starting_position() -> Game {
        Game {
            board: Board::starting_position(),
            to_move: Color::White,
            ep_square: None,
            castling_rights: CastlingRights::all(),
            fifty_move_count: 0,
            moves: 1,
        }
    }

    pub fn empty_position() -> Game {
        Game {
            board: Board::empty_position(),
            to_move: Color::White,
            ep_square: None,
            castling_rights: CastlingRights::empty(),
            fifty_move_count: 0,
            moves: 1
        }
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
                    '8' => decrement_square(&mut current_square, 8),
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

        match words[4].parse::<u32>() {
            Err(_) => return None,
            Ok(x) => game.fifty_move_count = x
        }

        match words[5].parse::<u32>() {
            Err(_) => return None,
            Ok(x) => game.moves = x
        }

        return Some(game);
    }

    pub fn generate_moves(&self, move_buffer: &mut Vec<Move>) {
        move_buffer.clear();

        use Color::*;
        use PieceType::*;

        let friendly_color   = self.to_move;
        let opponent_color  = !friendly_color;

        let empty_squares   = self.board.unoccupied();
        let all_pieces      = self.board.occupied();
        let friendly_pieces = self.board.occupied_by(friendly_color);
        let opponent_pieces = self.board.occupied_by(!friendly_color);

        let king_square     = self.board.get_king_square(friendly_color);
        let king_attackers = self.board.attackers(king_square, opponent_color);
        let checkers = king_attackers.population();
        let in_check = checkers > 0;
        let king_moves = KING_TABLE[king_square.idx()];
        let king_danger_squares = self.board.attacked(!friendly_color, true);

        if checkers > 1 {
            for to in king_moves & empty_squares & !king_danger_squares {
                move_buffer.push(Move::new(king_square, to, QUIET_FLAG));
            }
            return;
        }

        let mut capture_mask = opponent_pieces;
        let mut quiet_mask = empty_squares;

        if checkers == 1 {
            capture_mask = king_attackers;

            let checker_square = king_attackers.bitscan_forward();
            if self.board.piece_at(checker_square).unwrap().ptype.is_slider() {
                quiet_mask = ray_between_squares(king_square, checker_square);
            } else {
                quiet_mask = Bitboard::new(0);
            }
        }

        let not_king_bit = !king_square.bitrep();

        let mut pinned_diagonally = Bitboard::new(0);
        let mut pinned_nondiagonally = Bitboard::new(0);
        {
            let opRQ = self.board.get_pieces(opponent_color, Rook) | self.board.get_pieces(opponent_color, Queen);
            let mut pinner = xray_rook_attacks(all_pieces, friendly_pieces, king_square) & opRQ;
            for pinner_square in pinner {
                let connecting_bits = ray_between_squares(king_square, pinner_square);
                let pinned_bit = connecting_bits & friendly_pieces;
                assert!(pinned_bit.population() == 1);
                pinned_nondiagonally |= pinned_bit;
            }

            let opBQ = self.board.get_pieces(opponent_color, Bishop) | self.board.get_pieces(opponent_color, Queen);
            pinner = xray_bishop_attacks(all_pieces, friendly_pieces, king_square) & opBQ;
            for pinner_square in pinner {
                let connecting_bits = ray_between_squares(king_square, pinner_square);
                let pinned_bit = connecting_bits & friendly_pieces;
                assert!(pinned_bit.population() == 1);
                pinned_diagonally |= pinned_bit;
            }
        }

        let pinned = pinned_diagonally | pinned_nondiagonally;

        // MOVE GENERATION FOR UNPINNED PIECES

        /*********/
        /* PAWNS */
        /*********/
        {
            let unpinned_pawns = self.board.get_pieces(friendly_color, Pawn) & !pinned;
            let advanced_pawns = if friendly_color == White { unpinned_pawns.shifted_up() } else { unpinned_pawns.shifted_down() };
            let double_advanced_pawns = if friendly_color == White { advanced_pawns.shifted_up() } else { advanced_pawns.shifted_down() };
            let delta_pawn_single_push: i32 = if self.to_move == White { -8 } else { 8 };
            let delta_pawn_double_push: i32 = if self.to_move == White { -16 } else { 16 };
            let double_pawn_push_rank = if self.to_move == White { RANK4 } else { RANK5 };
            let promotion_rank = if self.to_move == White { 8 } else { 1 };

            // single pushes (and promotions)
            for to in advanced_pawns & empty_squares & quiet_mask
            {
                let from = Square::new((to.unwrap() as i32 + delta_pawn_single_push) as u32);

                if to.rank() == promotion_rank {
                    move_buffer.push(Move::new(from, to, BISHOP_PROMO_FLAG));
                    move_buffer.push(Move::new(from, to, KNIGHT_PROMO_FLAG));
                    move_buffer.push(Move::new(from, to, ROOK_PROMO_FLAG));
                    move_buffer.push(Move::new(from, to, QUEEN_PROMO_FLAG));
                } else {
                    move_buffer.push(Move::new(from, to, QUIET_FLAG));
                }
            }

            // double pushes
            for to in double_advanced_pawns & empty_squares & double_pawn_push_rank & quiet_mask {

                let from = Square::new((to.unwrap() as i32 + delta_pawn_double_push) as u32);

                move_buffer.push(Move::new(from, to, DOUBLE_PAWN_PUSH_FLAG));
            }

            // captures (and capture-promotions)
            for from in unpinned_pawns
            {
                let pawn_attack_pattern = PAWN_ATTACKS[friendly_color as usize][from.idx()] & capture_mask;

                for to in pawn_attack_pattern & opponent_pieces
                {
                    if to.rank() == promotion_rank {
                        move_buffer.push(Move::new(from, to, BISHOP_PROMO_CAPTURE_FLAG));
                        move_buffer.push(Move::new(from, to, KNIGHT_PROMO_CAPTURE_FLAG));
                        move_buffer.push(Move::new(from, to, ROOK_PROMO_CAPTURE_FLAG));
                        move_buffer.push(Move::new(from, to, QUEEN_PROMO_CAPTURE_FLAG));
                    } else {
                        move_buffer.push(Move::new(from, to, CAPTURE_FLAG));
                    }
                }

                match self.ep_square {
                    Some(sq) =>
                        if (pawn_attack_pattern & sq.bitrep()).nonempty() {
                            move_buffer.push(Move::new(from, sq, EP_CAPTURE_FLAG));
                        },
                    None => {}
                }
            }
        }

        /***********/
        /* KNIGHTS */
        /***********/
        {
            for from in self.board.get_pieces(friendly_color, Knight) & !pinned
            {
                let knight_moves = KNIGHT_TABLE[from.idx()];

                /* quiets */
                for to in knight_moves & empty_squares & quiet_mask {
                    move_buffer.push(Move::new(from, to, QUIET_FLAG));
                }

                /* captures */
                for to in knight_moves & opponent_pieces & capture_mask {
                    move_buffer.push(Move::new(from, to, CAPTURE_FLAG));
                }
            }
        }

        /***********/
        /* BISHOPS */
        /***********/

        for from in self.board.get_pieces(friendly_color, Bishop) & !pinned
        {
            let bishop_moves = get_bishop_rays(from, all_pieces);

            /* quiets */
            for to in bishop_moves & empty_squares & quiet_mask {
                move_buffer.push(Move::new(from, to, QUIET_FLAG));
            }

            /* captures */
            for to in bishop_moves & opponent_pieces & capture_mask {
                move_buffer.push(Move::new(from, to, CAPTURE_FLAG));
            }
        }

        /*********/
        /* ROOKS */
        /*********/

        for from in self.board.get_pieces(friendly_color, Rook) & !pinned
        {
            let rook_moves = get_rook_rays(from, all_pieces);

            /* quiets */
            for to in rook_moves & empty_squares & quiet_mask {
                move_buffer.push(Move::new(from, to, QUIET_FLAG));
            }

            /* captures */
            for to in rook_moves & opponent_pieces & capture_mask {
                move_buffer.push(Move::new(from, to, CAPTURE_FLAG));
            }
        }

        /*********/
        /* QUEEN */
        /*********/

        for from in self.board.get_pieces(friendly_color, Queen) & !pinned
        {
            let queen_moves = get_queen_rays(from, all_pieces);

            /* quiets */
            for to in queen_moves & empty_squares & quiet_mask {
                move_buffer.push(Move::new(from, to, QUIET_FLAG));
            }

            /* captures */
            for to in queen_moves & opponent_pieces & capture_mask {
                move_buffer.push(Move::new(from, to, CAPTURE_FLAG));
            }
        }

        /********/
        /* KING */
        /********/

        {
            /* quiets */
            for to in king_moves & empty_squares & !king_danger_squares {
                move_buffer.push(Move::new(king_square, to, QUIET_FLAG));
            }

            /* captures */
            for to in king_moves & opponent_pieces & capture_mask {
                move_buffer.push(Move::new(king_square, to, CAPTURE_FLAG));
            }

            /* castling */
            let has_kingside_castle_rights = self.castling_rights.intersects(CastlingRights::WHITE_KINGSIDE);
            let has_queenside_castle_rights = self.castling_rights.intersects(CastlingRights::WHITE_QUEENSIDE);

            if has_kingside_castle_rights && !in_check {
                let kingside_castle_path_open = (all_pieces & WHITE_KINGSIDE_CASTLE_BITS).empty();

                if kingside_castle_path_open {
                    let mut castle_path_is_safe: bool = true;

                    if (WHITE_KINGSIDE_CASTLE_BITS & king_danger_squares).nonempty() {
                        castle_path_is_safe = false;
                    }

                    if castle_path_is_safe {
                        move_buffer.push(Move::new(king_square, Square::new(1), KING_CASTLE_FLAG));
                    }
                }
            }

            if has_queenside_castle_rights && !in_check {
                let queenside_castle_path_open = (all_pieces & WHITE_QUEENSIDE_CASTLE_BITS).empty();

                if queenside_castle_path_open {
                    let mut castle_path_is_safe: bool = true;


                    if (WHITE_QUEENSIDE_CASTLE_BITS & king_danger_squares).nonempty() {
                        castle_path_is_safe = false;
                    }

                    if castle_path_is_safe {
                        move_buffer.push(Move::new(king_square, Square::new(5), QUEEN_CASTLE_FLAG));
                    }
                }
            }
        }
    }
}
