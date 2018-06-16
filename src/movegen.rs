use core::*;
use bitboard::*;
use moves::*;
use board::*;
use tables::*;
use game::*;
use pins::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::cell::RefMut;

#[derive(Clone, Copy)]
pub struct MoveGen {
    pin_finder: PinFinder
}

impl MoveGen {
    pub fn new() -> MoveGen {
        MoveGen {
            pin_finder: PinFinder::new()
        }
    }

    pub fn next_states(game: &Game) -> Vec<(Move, Game)> {
        let mut states = Vec::new();

        let mut move_gen = MoveGen::new();
        let move_buffer = alloc_move_buffer();
        move_gen.fill_move_buffer(&game, &move_buffer);

        for m in move_buffer.borrow().iter() {
            let mut game_copy = game.clone();
            game_copy.make_move(*m);
            states.push( (*m, game_copy.clone()) );
        }

        return states;
    }

    #[allow(dead_code)]
    pub fn next_states_chunked(game: &Game, num_cpu_cores: usize) -> Vec<Vec<(Move, Game)>> {
        let states = MoveGen::next_states(game);
        let mut chunks = Vec::new();

        for _ in 0 .. num_cpu_cores {
            chunks.push(Vec::new());
        }

        let mut i = 0;
        for x in states.iter() {
            chunks[i % num_cpu_cores].push(x.clone());
            i += 1;
        }

        return chunks;
    }

    pub fn fill_move_buffer(&mut self, game: &Game,
                            move_buffer_ref: &Rc<RefCell<MoveList>>)
    {
        let mut move_buffer = move_buffer_ref.borrow_mut();
        move_buffer.clear();

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
                move_buffer.add(Move::new(king_square, to, QUIET_FLAG));
            }
            for to in king_moves & opponent_pieces & !king_danger_squares {
                move_buffer.add(Move::new(king_square, to, CAPTURE_FLAG));
            }
            return;
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

        self.pin_finder.update(friendly_color, &game.board);
        let pinned_diagonally = self.pin_finder.pinned_diagonally();
        let pinned_nondiagonally = self.pin_finder.pinned_nondiagonally();
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

                // todo: don't do inner loop test, just separate move generation for pinned pawns.
                if (from.bitrep() & pinned_nondiagonally).nonempty()
                    && (to.bitrep() & self.pin_finder.nondiagonal_constraint(from)).empty() {
                        continue;
                }

                if to.rank() == promotion_rank {
                    move_buffer.add(Move::new(from, to, BISHOP_PROMO_FLAG));
                    move_buffer.add(Move::new(from, to, KNIGHT_PROMO_FLAG));
                    move_buffer.add(Move::new(from, to, ROOK_PROMO_FLAG));
                    move_buffer.add(Move::new(from, to, QUEEN_PROMO_FLAG));
                } else {
                    move_buffer.add(Move::new(from, to, QUIET_FLAG));
                }
            }

            // double pushes
            for to in double_advanced_pawns & empty_squares & double_pawn_push_rank & quiet_mask {
                let from = Square::new((to.unwrap() as i32 + delta_pawn_double_push) as u32);

                if (from.bitrep() & pinned_nondiagonally).nonempty()
                    && (to.bitrep() & self.pin_finder.nondiagonal_constraint(from)).empty() {
                        continue;
                }

                move_buffer.add(Move::new(from, to, DOUBLE_PAWN_PUSH_FLAG));
            }

            let pawns_that_can_capture = friendly_pawns & !pinned_nondiagonally;

            // captures (and capture-promotions)
            for from in pawns_that_can_capture
            {
                let mut pawn_attack_pattern = PAWN_ATTACKS[friendly_color as usize][from.idx()] & capture_mask;
                if (from.bitrep() & pinned_diagonally).nonempty() {
                    pawn_attack_pattern &= self.pin_finder.diagonal_constraint(from);
                }

                for to in pawn_attack_pattern & opponent_pieces
                {
                    if to.rank() == promotion_rank {
                        move_buffer.add(Move::new(from, to, BISHOP_PROMO_CAPTURE_FLAG));
                        move_buffer.add(Move::new(from, to, KNIGHT_PROMO_CAPTURE_FLAG));
                        move_buffer.add(Move::new(from, to, ROOK_PROMO_CAPTURE_FLAG));
                        move_buffer.add(Move::new(from, to, QUEEN_PROMO_CAPTURE_FLAG));
                    } else {
                        move_buffer.add(Move::new(from, to, CAPTURE_FLAG));
                    }
                }

                match game.ep_square {
                    //TODO: en-passante discovered check test
                    Some(ep_capture_square) => {
                        let captured_sq = match opponent_color {
                                White => Square::new(ep_capture_square.unwrap() + 8),
                                Black => Square::new(ep_capture_square.unwrap() - 8)
                        };

                        if (captured_sq.bitrep() & capture_mask).nonempty()
                            && (PAWN_ATTACKS[friendly_color as usize][from.idx()] & ep_capture_square.bitrep()).nonempty()
                        {
                            let mut board_copy = game.board.clone();

                            *board_copy.get_pieces_mut(opponent_color, Pawn) &= !captured_sq.bitrep();
                            *board_copy.get_pieces_mut(friendly_color, Pawn) ^= from.bitrep() | ep_capture_square.bitrep();
                            *board_copy.occupied_by_mut(opponent_color) &= !captured_sq.bitrep();
                            *board_copy.occupied_by_mut(friendly_color) ^= from.bitrep() | ep_capture_square.bitrep();

                            let attackers = board_copy.attackers(king_square, opponent_color);
                            if attackers.empty() {
                                move_buffer.add(Move::new(from, ep_capture_square, EP_CAPTURE_FLAG));
                            }
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
                    move_buffer.add(Move::new(from, to, QUIET_FLAG));
                }

                /* captures */
                for to in knight_moves & opponent_pieces & capture_mask {
                    move_buffer.add(Move::new(from, to, CAPTURE_FLAG));
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
                    move_buffer.add(Move::new(from, to, QUIET_FLAG));
                }

                // captures
                for to in bishop_moves & opponent_pieces & capture_mask {
                    move_buffer.add(Move::new(from, to, CAPTURE_FLAG));
                }
            }

            // PINNED
            for from in friendly_bishops & pinned_diagonally
            {
                let bishop_moves = get_bishop_rays(from, occupied_squares)
                                   & self.pin_finder.diagonal_constraint(from);

                // quiets
                for to in bishop_moves & empty_squares & quiet_mask {
                    move_buffer.add(Move::new(from, to, QUIET_FLAG));
                }

                // captures
                for to in bishop_moves & opponent_pieces & capture_mask {
                    move_buffer.add(Move::new(from, to, CAPTURE_FLAG));
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
                    move_buffer.add(Move::new(from, to, QUIET_FLAG));
                }

                /* captures */
                for to in rook_moves & opponent_pieces & capture_mask {
                    move_buffer.add(Move::new(from, to, CAPTURE_FLAG));
                }
            }

            // pinned
            for from in friendly_rooks & pinned_nondiagonally
            {
                let rook_moves = get_rook_rays(from, occupied_squares)
                                 & self.pin_finder.nondiagonal_constraint(from);

                /* quiets */
                for to in rook_moves & empty_squares & quiet_mask {
                    move_buffer.add(Move::new(from, to, QUIET_FLAG));
                }

                /* captures */
                for to in rook_moves & opponent_pieces & capture_mask {
                    move_buffer.add(Move::new(from, to, CAPTURE_FLAG));
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
                    move_buffer.add(Move::new(from, to, QUIET_FLAG));
                }

                /* captures */
                for to in queen_moves & opponent_pieces & capture_mask {
                    move_buffer.add(Move::new(from, to, CAPTURE_FLAG));
                }
            }

            let movable_pinned_queens = friendly_queens & pinned & !(pinned_diagonally & pinned_nondiagonally);

            for from in movable_pinned_queens & pinned_diagonally
            {
                let queen_moves = get_queen_rays(from, occupied_squares)
                                  & self.pin_finder.diagonal_constraint(from);

                /* quiets */
                for to in queen_moves & empty_squares & quiet_mask {
                    move_buffer.add(Move::new(from, to, QUIET_FLAG));
                }

                /* captures */
                for to in queen_moves & opponent_pieces & capture_mask {
                    move_buffer.add(Move::new(from, to, CAPTURE_FLAG));
                }
            }

            for from in movable_pinned_queens & pinned_nondiagonally
            {
                let queen_moves = get_queen_rays(from, occupied_squares)
                                  & self.pin_finder.nondiagonal_constraint(from);

                /* quiets */
                for to in queen_moves & empty_squares & quiet_mask {
                    move_buffer.add(Move::new(from, to, QUIET_FLAG));
                }

                /* captures */
                for to in queen_moves & opponent_pieces & capture_mask {
                    move_buffer.add(Move::new(from, to, CAPTURE_FLAG));
                }
            }
        }

        /********/
        /* KING */
        /********/

        {
            /* quiets */
            for to in king_moves & empty_squares & !king_danger_squares {
                move_buffer.add(Move::new(king_square, to, QUIET_FLAG));
            }

            /* captures */
            for to in king_moves & opponent_pieces & !king_danger_squares {
                move_buffer.add(Move::new(king_square, to, CAPTURE_FLAG));
            }

            /* castling */
            let has_kingside_castle_rights = match friendly_color {
                White => game.castling_rights.intersects(CastlingRights::WHITE_KINGSIDE),
                Black => game.castling_rights.intersects(CastlingRights::BLACK_KINGSIDE)
            };

            let has_queenside_castle_rights = match friendly_color {
                White => game.castling_rights.intersects(CastlingRights::WHITE_QUEENSIDE),
                Black => game.castling_rights.intersects(CastlingRights::BLACK_QUEENSIDE)
            };

            if has_kingside_castle_rights && !in_check {
                let kingside_bits = match friendly_color {
                    White => WHITE_KINGSIDE_CASTLE_BITS,
                    Black => BLACK_KINGSIDE_CASTLE_BITS
                };

                let kingside_castle_path_open = (occupied_squares & kingside_bits).empty();

                if kingside_castle_path_open {
                    let mut castle_path_is_safe: bool = true;

                    if (kingside_bits & king_danger_squares).nonempty() {
                        castle_path_is_safe = false;
                    }

                    if castle_path_is_safe {
                        match friendly_color {
                            White => move_buffer.add(Move::new(king_square, Square::new(1), KING_CASTLE_FLAG)),
                            Black => move_buffer.add(Move::new(king_square, Square::new(57), KING_CASTLE_FLAG))
                        }
                    }
                }
            }

            if has_queenside_castle_rights && !in_check {
                let queenside_path_bits = match friendly_color {
                    White => WHITE_QUEENSIDE_CASTLE_BITS,
                    Black => BLACK_QUEENSIDE_CASTLE_BITS
                };

                let queenside_safety_bits = match friendly_color {
                    White => WHITE_QUEENSIDE_CASTLE_SAFETY_BITS,
                    Black => BLACK_QUEENSIDE_CASTLE_SAFETY_BITS
                };

                let queenside_castle_path_open = (occupied_squares & queenside_path_bits).empty();

                if queenside_castle_path_open {
                    let mut castle_path_is_safe: bool = true;

                    if (queenside_safety_bits & king_danger_squares).nonempty() {
                        castle_path_is_safe = false;
                    }

                    if castle_path_is_safe {
                        match friendly_color {
                            White => move_buffer.add(Move::new(king_square, Square::new(5), QUEEN_CASTLE_FLAG)),
                            Black => move_buffer.add(Move::new(king_square, Square::new(61), QUEEN_CASTLE_FLAG))
                        }
                    }
                }
            }
        }
    }
}

pub fn move_from_algebraic(game: &Game, move_str: String) -> Option<Move> {
    let from_str = move_str[..2].to_string();
    let to_str = move_str[2..].to_string();

    match Square::from_algebraic(&from_str) {
        Some(from_sq) => {
            match Square::from_algebraic(&to_str) {
                Some(to_sq) => {
                    let mut move_gen = MoveGen::new();
                    let move_buffer = alloc_move_buffer();
                    move_gen.fill_move_buffer(&game, &move_buffer);
                    for m in move_buffer.borrow().iter() {
                        if m.from() == from_sq && m.to() == to_sq {
                            return Some(*m);
                        }
                    }
                }
                None => {
                    println!("invalid to string: {}", to_str);
                    return None;
                }
            }
        },
        None => {
            println!("invalid from string: {}", from_str);
            return None;
        }
    }

    return None;
}

