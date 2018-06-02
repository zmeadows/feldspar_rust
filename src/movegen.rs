use core::*;
use bitboard::*;
use moves::*;
use board::*;
use tables::*;
use game::*;

pub struct MoveGen {
    diag_pin_map: [Bitboard; 64],
    nondiag_pin_map: [Bitboard; 64]
}

impl MoveGen {
    pub fn new() -> MoveGen {
        return MoveGen { diag_pin_map: [Bitboard::new(0); 64], nondiag_pin_map: [Bitboard::new(0); 64] };
    }

    pub fn move_list(&mut self, game: &Game) -> Vec<Move> {
        let mut move_buffer = Vec::new();

        use Color::*;
        use PieceType::*;

        let friendly_color  = game.to_move;
        let opponent_color  = !friendly_color;

        let empty_squares    = game.board.unoccupied();
        let occupied_squares = game.board.occupied();
        let friendly_pieces  = game.board.occupied_by(friendly_color);
        let opponent_pieces  = game.board.occupied_by(!friendly_color);

        let king_square         = game.board.get_king_square(friendly_color);
        let king_attackers      = game.board.attackers(king_square, opponent_color);
        let check_multiplicity  = king_attackers.population();
        let in_check            = check_multiplicity > 0;
        let king_moves          = KING_TABLE[king_square.idx()];
        let king_danger_squares = game.board.attacked(!friendly_color, true);

        if check_multiplicity > 1 {
            for to in king_moves & empty_squares & !king_danger_squares {
                move_buffer.push(Move::new(king_square, to, QUIET_FLAG));
            }
            return move_buffer;
        }

        let mut capture_mask = Bitboard::new(u64::max_value());
        let mut quiet_mask = Bitboard::new(u64::max_value());

        if check_multiplicity == 1 {
            capture_mask = king_attackers;

            let checker_square = king_attackers.bitscan_forward();
            if game.board.piece_at(checker_square).unwrap().ptype.is_slider() {
                quiet_mask = ray_between_squares(king_square, checker_square);
            } else {
                quiet_mask = Bitboard::new(0);
            }
        }

        let mut pinned_diagonally = Bitboard::new(0);
        let mut pinned_nondiagonally = Bitboard::new(0);

        // NOTE: stored *_pin_map arrays could potentially be a source of strange bugs.
        // NOTE: easy to debug though, just clear them each call and see if results change.

        {
            let opRQ = game.board.get_pieces(opponent_color, Rook) | game.board.get_pieces(opponent_color, Queen);
            let mut pinner = xray_rook_attacks(occupied_squares, friendly_pieces, king_square) & opRQ;
            for pinner_square in pinner {
                let connecting_bits = ray_between_squares(king_square, pinner_square);
                let pinned_bit = connecting_bits & friendly_pieces;
                self.nondiag_pin_map[pinned_bit.bitscan_forward().idx()] = connecting_bits;
                assert!(pinned_bit.population() == 1);
                pinned_nondiagonally |= pinned_bit;
            }

            let opBQ = game.board.get_pieces(opponent_color, Bishop) | game.board.get_pieces(opponent_color, Queen);
            pinner = xray_bishop_attacks(occupied_squares, friendly_pieces, king_square) & opBQ;
            for pinner_square in pinner {
                let connecting_bits = ray_between_squares(king_square, pinner_square);
                let pinned_bit = connecting_bits & friendly_pieces;
                self.diag_pin_map[pinned_bit.bitscan_forward().idx()] = connecting_bits;
                assert!(pinned_bit.population() == 1);
                pinned_diagonally |= pinned_bit;
            }
        }

        let pinned = pinned_diagonally | pinned_nondiagonally;

        let friendly_pawns = game.board.get_pieces(friendly_color, Pawn);
        let delta_pawn_single_push: i32 = if game.to_move == White { -8 } else { 8 };
        let delta_pawn_double_push: i32 = if game.to_move == White { -16 } else { 16 };
        let double_pawn_push_rank = if game.to_move == White { RANK4 } else { RANK5 };
        let promotion_rank = if game.to_move == White { 8 } else { 1 };

        /*********/
        /* PAWNS */
        /*********/
        {
            let advanceable_pawns = friendly_pawns & !pinned_diagonally;

            let advanced_pawns =
                if friendly_color == White {
                    advanceable_pawns.shifted_up() & empty_squares
                } else {
                    advanceable_pawns.shifted_down() & empty_squares
                };

            let double_advanced_pawns =
                if friendly_color == White {
                    advanced_pawns.shifted_up()
                } else {
                    advanced_pawns.shifted_down()
                };

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

            let pawns_that_can_capture = friendly_pawns & !pinned_nondiagonally;

            // captures (and capture-promotions)
            for from in pawns_that_can_capture
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

                match game.ep_square {
                    //TODO: en-passante discovered check test
                    Some(sq) => {
                        if (pawn_attack_pattern & sq.bitrep()).nonempty() {
                            move_buffer.push(Move::new(from, sq, EP_CAPTURE_FLAG));
                        }
                    },
                    None => {}
                }
            }
        }

        /***********/
        /* KNIGHTS */
        /***********/
        {
            for from in game.board.get_pieces(friendly_color, Knight) & !pinned
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

        {
            let friendly_bishops = game.board.get_pieces(friendly_color, Bishop);

            // UNPINNED
            for from in friendly_bishops & !pinned
            {
                let bishop_moves = get_bishop_rays(from, occupied_squares);

                // quiets
                for to in bishop_moves & empty_squares & quiet_mask {
                    move_buffer.push(Move::new(from, to, QUIET_FLAG));
                }

                // captures
                for to in bishop_moves & opponent_pieces & capture_mask {
                    move_buffer.push(Move::new(from, to, CAPTURE_FLAG));
                }
            }

            // PINNED
            for from in friendly_bishops & pinned_diagonally
            {
                let bishop_moves = get_bishop_rays(from, occupied_squares) & self.diag_pin_map[from.idx()];

                // quiets
                for to in bishop_moves & empty_squares & quiet_mask {
                    move_buffer.push(Move::new(from, to, QUIET_FLAG));
                }

                // captures
                for to in bishop_moves & opponent_pieces & capture_mask {
                    move_buffer.push(Move::new(from, to, CAPTURE_FLAG));
                }
            }

        }

        /*********/
        /* ROOKS */
        /*********/

        {
            let friendly_rooks = game.board.get_pieces(friendly_color, Rook);

            // unpinned
            for from in friendly_rooks & !pinned
            {
                let rook_moves = get_rook_rays(from, occupied_squares);

                /* quiets */
                for to in rook_moves & empty_squares & quiet_mask {
                    move_buffer.push(Move::new(from, to, QUIET_FLAG));
                }

                /* captures */
                for to in rook_moves & opponent_pieces & capture_mask {
                    move_buffer.push(Move::new(from, to, CAPTURE_FLAG));
                }
            }

            // pinned
            for from in friendly_rooks & pinned_nondiagonally
            {
                let rook_moves = get_rook_rays(from, occupied_squares) & self.nondiag_pin_map[from.idx()];

                /* quiets */
                for to in rook_moves & empty_squares & quiet_mask {
                    move_buffer.push(Move::new(from, to, QUIET_FLAG));
                }

                /* captures */
                for to in rook_moves & opponent_pieces & capture_mask {
                    move_buffer.push(Move::new(from, to, CAPTURE_FLAG));
                }
            }
        }

        /*********/
        /* QUEEN */
        /*********/
        {
            let friendly_queens = game.board.get_pieces(friendly_color, Queen);

            for from in friendly_queens & !pinned
            {
                let queen_moves = get_queen_rays(from, occupied_squares);

                /* quiets */
                for to in queen_moves & empty_squares & quiet_mask {
                    move_buffer.push(Move::new(from, to, QUIET_FLAG));
                }

                /* captures */
                for to in queen_moves & opponent_pieces & capture_mask {
                    move_buffer.push(Move::new(from, to, CAPTURE_FLAG));
                }
            }

            for from in friendly_queens & pinned
            {
                let queen_moves = get_queen_rays(from, occupied_squares)
                    & self.nondiag_pin_map[from.idx()] & self.diag_pin_map[from.idx()];

                /* quiets */
                for to in queen_moves & empty_squares & quiet_mask {
                    move_buffer.push(Move::new(from, to, QUIET_FLAG));
                }

                /* captures */
                for to in queen_moves & opponent_pieces & capture_mask {
                    move_buffer.push(Move::new(from, to, CAPTURE_FLAG));
                }
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
            let has_kingside_castle_rights = game.castling_rights.intersects(CastlingRights::WHITE_KINGSIDE);
            let has_queenside_castle_rights = game.castling_rights.intersects(CastlingRights::WHITE_QUEENSIDE);

            if has_kingside_castle_rights && !in_check {
                let kingside_castle_path_open = (occupied_squares & WHITE_KINGSIDE_CASTLE_BITS).empty();

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
                let queenside_castle_path_open = (occupied_squares & WHITE_QUEENSIDE_CASTLE_BITS).empty();

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

        return move_buffer;
    }
}
