use core::*;
use moves::*;
use tables::*;
use pins::*;
use game::*;
use move_list::*;
use bitboard::*;

use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum MoveGenStage {
    Begin,
    MultipleCheckKing,
    Knight,
    Bishop,
    Rook,
    Queen,
    PawnQuiet,
    PawnCapture,
    KingNonCastle,
    Finished
}

#[derive(Clone, Copy)]
pub struct MoveBufferData {
    pub list: MoveList,
    pub stage: MoveGenStage,
    pub pin_finder: PinFinder
}

pub type MoveBuffer = Rc<RefCell<MoveBufferData>>;

pub fn alloc_move_buffer() -> MoveBuffer {
    Rc::new(RefCell::new(MoveBufferData::new()))
}

pub fn next_moves_standalone(game: &Game) -> MoveList {
    let buf = alloc_move_buffer();
    buf.borrow_mut().generate_moves(&game, false);
    return buf.borrow().list;
}

pub fn next_moves_standalone_chunked(game: &Game, chunks: usize) -> Vec<Vec<Move>> {
    let buf = alloc_move_buffer();
    buf.borrow_mut().generate_moves(&game, false);

    let mut move_chunks = Vec::new();

    for (i, m) in buf.borrow().list.iter().enumerate() {
        if move_chunks.len() <= i {
            move_chunks.push(Vec::new());
        }

        move_chunks[i % chunks].push(*m);
    }

    return move_chunks;
}

impl MoveBufferData {
    pub fn new() -> MoveBufferData {
        MoveBufferData {
            list: MoveList::new(),
            stage: MoveGenStage::Begin,
            pin_finder: PinFinder::new()
        }
    }

    pub fn clear(&mut self) {
        self.list.clear();
        self.stage = MoveGenStage::Begin;
    }

    // returns true if any moves are found
    pub fn can_move(&mut self, game: &Game) -> bool {
        use Color::*;
        use PieceType::*;

        // OPTIMIZE: check if any of these can be moved below
        let friendly_color      = game.to_move;
        let opponent_color      = !friendly_color;
        let empty_squares       = game.board.unoccupied();
        let occupied_squares    = game.board.occupied();
        let friendly_pieces     = game.board.occupied_by(friendly_color);
        let opponent_pieces     = game.board.occupied_by(!friendly_color);
        let king_square         = game.board.get_king_square(friendly_color);
        let king_attackers      = game.king_attackers;
        let check_multiplicity  = king_attackers.population();
        let in_check            = check_multiplicity > 0;

        let opponent_pawns = game.board.get_pieces(opponent_color, Pawn);
        let opponent_knights = game.board.get_pieces(opponent_color, Knight);
        let opponent_bishops = game.board.get_pieces(opponent_color, Bishop);
        let opponent_rooks = game.board.get_pieces(opponent_color, Rook);
        let opponent_queens = game.board.get_pieces(opponent_color, Queen);
        let opponent_kings = game.board.get_pieces(opponent_color, King);

        let opp_ptype_at = move |sq: Square| -> PieceType {
            use PieceType::*;

            let sqbit = sq.bitrep();

            if (sqbit & opponent_pawns).nonempty() {
                return Pawn;
            } else if (sqbit & opponent_knights).nonempty() {
                return Knight;
            } else if (sqbit & opponent_bishops).nonempty() {
                return Bishop;
            } else if (sqbit & opponent_rooks).nonempty() {
                return Rook;
            } else if (sqbit & opponent_queens).nonempty() {
                return Queen;
            } else if (sqbit & opponent_kings).nonempty() {
                return King;
            } else {
                panic!("Attempted to determine opponent piece type at an empty square.");
            }
        };

        let king_moves = KING_TABLE[king_square.idx()];

        if check_multiplicity > 1 && self.stage < MoveGenStage::MultipleCheckKing {
            // If the king is in double+ check, the only legal moves are
            // king moves, so we compute them and return early.
            let king_danger_squares = game.board.attacked(!game.to_move, true);

            if (king_moves & empty_squares & !king_danger_squares).nonempty() {
                return true;
            }

            if (king_moves & opponent_pieces & !king_danger_squares).nonempty() {
                return true;
            }

            return false;
        }

        let mut capture_mask = Bitboard::new(u64::max_value());
        let mut quiet_mask = Bitboard::new(u64::max_value());

        if check_multiplicity == 1 {
            capture_mask = king_attackers;

            let checker_square = king_attackers.bitscan_forward();

            if opp_ptype_at(checker_square).is_slider() {
                quiet_mask = ray_between_squares(king_square, checker_square);
            } else {
                quiet_mask = Bitboard::new(0);
            }
        }

        self.pin_finder.update(friendly_color, &game.board);
        let pinned_diagonally = self.pin_finder.pinned_diagonally();
        let pinned_nondiagonally = self.pin_finder.pinned_nondiagonally();
        let pinned = self.pin_finder.pinned();

        /***********/
        /* KNIGHTS */
        /***********/
        {
            for from in game.board.get_pieces(friendly_color, Knight) & !pinned
            {
                let knight_moves = KNIGHT_TABLE[from.idx()];

                if (knight_moves & empty_squares & quiet_mask).nonempty() {
                    return true;
                }

                if (knight_moves & opponent_pieces & capture_mask).nonempty() {
                    return true;
                }
            }
        } // end knights

        /***********/
        /* BISHOPS */
        /***********/


        {
            let friendly_bishops = game.board.get_pieces(friendly_color, Bishop);

            // UNPINNED
            for from in friendly_bishops & !pinned
            {
                let bishop_moves = get_bishop_rays(from, occupied_squares);

                if (bishop_moves & empty_squares & quiet_mask).nonempty() {
                    return true;
                }

                if (bishop_moves & opponent_pieces & capture_mask).nonempty() {
                    return true;
                }
            }

            // PINNED
            for from in friendly_bishops & pinned_diagonally
            {
                let bishop_moves = get_bishop_rays(from, occupied_squares)
                    & self.pin_finder.diagonal_constraint(from);

                if (bishop_moves & empty_squares & quiet_mask).nonempty() {
                    return true;
                }

                if (bishop_moves & opponent_pieces & capture_mask).nonempty() {
                    return true;
                }
            }
        } // end bishops

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
                if (rook_moves & empty_squares & quiet_mask).nonempty() {
                    return true;
                }

                /* captures */
                if (rook_moves & opponent_pieces & capture_mask).nonempty() {
                    return true;
                }
            }

            // pinned
            for from in friendly_rooks & pinned_nondiagonally
            {
                let rook_moves = get_rook_rays(from, occupied_squares)
                                 & self.pin_finder.nondiagonal_constraint(from);

                /* quiets */
                if (rook_moves & empty_squares & quiet_mask).nonempty() {
                    return true;
                }

                /* captures */
                if (rook_moves & opponent_pieces & capture_mask).nonempty() {
                    return true;
                }
            }
        } // end rooks

        /*********/
        /* QUEEN */
        /*********/

        {
            let friendly_queens = game.board.get_pieces(friendly_color, Queen);

            for from in friendly_queens & !pinned
            {
                let queen_moves = get_queen_rays(from, occupied_squares);

                /* quiets */
                if (queen_moves & empty_squares & quiet_mask).nonempty() {
                    return true;
                }

                /* captures */
                if (queen_moves & opponent_pieces & capture_mask).nonempty() {
                    return true;
                }
            }

            let movable_pinned_queens = friendly_queens & pinned & !(pinned_diagonally & pinned_nondiagonally);

            for from in movable_pinned_queens & pinned_diagonally
            {
                let queen_moves = get_queen_rays(from, occupied_squares)
                                  & self.pin_finder.diagonal_constraint(from);

                /* quiets */
                if (queen_moves & empty_squares & quiet_mask).nonempty() {
                    return true;
                }

                /* captures */
                if (queen_moves & opponent_pieces & capture_mask).nonempty() {
                    return true;
                }
            }

            for from in movable_pinned_queens & pinned_nondiagonally
            {
                let queen_moves = get_queen_rays(from, occupied_squares)
                                  & self.pin_finder.nondiagonal_constraint(from);

                /* quiets */
                if (queen_moves & empty_squares & quiet_mask).nonempty() {
                    return true;
                }

                /* captures */
                if (queen_moves & opponent_pieces & capture_mask).nonempty() {
                    return true;
                }
            }
        } // end queens

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

            // single pushes (and promotions)
            for to in advanced_pawns & empty_squares & quiet_mask
            {
                let from = Square::new((to.unwrap() as i32 + delta_pawn_single_push) as u32);

                // todo: don't do inner loop test, just separate move generation for pinned pawns.
                if (from.bitrep() & pinned_nondiagonally).nonempty()
                    && (to.bitrep() & self.pin_finder.nondiagonal_constraint(from)).empty() {
                        continue;
                } else {
                    return true;
                }
            }

            let double_advanced_pawns =
                if friendly_color == White {
                    advanced_pawns.shifted_up()
                } else {
                    advanced_pawns.shifted_down()
                };

            // double pushes
            for to in double_advanced_pawns & empty_squares & double_pawn_push_rank & quiet_mask {
                let from = Square::new((to.unwrap() as i32 + delta_pawn_double_push) as u32);

                if (from.bitrep() & pinned_nondiagonally).nonempty()
                    && (to.bitrep() & self.pin_finder.nondiagonal_constraint(from)).empty() {
                        continue;
                } else {
                    return true;
                }
            }
        }

        //TODO: fix this mess
        if self.stage < MoveGenStage::PawnCapture {
            let pawns_that_can_capture = friendly_pawns & !pinned_nondiagonally;

            // captures (and capture-promotions)
            for from in pawns_that_can_capture
            {
                let mut pawn_attack_pattern = PAWN_ATTACKS[friendly_color as usize][from.idx()] & capture_mask;

                if (from.bitrep() & pinned_diagonally).nonempty() {
                    pawn_attack_pattern &= self.pin_finder.diagonal_constraint(from);
                }

                if (pawn_attack_pattern & opponent_pieces).nonempty() {
                    return true;
                }


                match game.ep_square {
                    None => {}
                    Some(ep_capture_square) => {
                        let captured_sq = match opponent_color {
                            White => Square::new(ep_capture_square.unwrap() + 8),
                            Black => Square::new(ep_capture_square.unwrap() - 8)
                        };

                        //CLEANUP
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
                                return true;
                            }
                        }
                    }
                }
            }
        }


        /********/
        /* KING */
        /********/

        let king_danger_squares = game.board.attacked(!game.to_move, true);

        if self.stage < MoveGenStage::KingNonCastle {

            /* quiets */
            if (king_moves & empty_squares & !king_danger_squares).nonempty() {
                return true;
            }

            /* captures */
            if (king_moves & opponent_pieces & !king_danger_squares).nonempty() {
                return true;
            }
        }

        /* castling */
        if self.stage < MoveGenStage::Finished {
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
                        return true;
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
                        return true;
                    }
                }
            }

            self.stage = MoveGenStage::Finished;
        }

        return false;
    }

    // returns true if any moves are found
    pub fn generate_moves(&mut self, game: &Game, break_on_found: bool) -> bool {
        use Color::*;
        use PieceType::*;

        // OPTIMIZE: check if any of these can be moved below
        let friendly_color      = game.to_move;
        let opponent_color      = !friendly_color;
        let empty_squares       = game.board.unoccupied();
        let occupied_squares    = game.board.occupied();
        let friendly_pieces     = game.board.occupied_by(friendly_color);
        let opponent_pieces     = game.board.occupied_by(!friendly_color);
        let king_square         = game.board.get_king_square(friendly_color);
        let king_attackers      = game.king_attackers;
        let check_multiplicity  = king_attackers.population();
        let in_check            = check_multiplicity > 0;

        let opponent_pawns = game.board.get_pieces(opponent_color, Pawn);
        let opponent_knights = game.board.get_pieces(opponent_color, Knight);
        let opponent_bishops = game.board.get_pieces(opponent_color, Bishop);
        let opponent_rooks = game.board.get_pieces(opponent_color, Rook);
        let opponent_queens = game.board.get_pieces(opponent_color, Queen);
        let opponent_kings = game.board.get_pieces(opponent_color, King);

        let opp_ptype_at = move |sq: Square| -> PieceType {
            use PieceType::*;

            let sqbit = sq.bitrep();

            if (sqbit & opponent_pawns).nonempty() {
                return Pawn;
            } else if (sqbit & opponent_knights).nonempty() {
                return Knight;
            } else if (sqbit & opponent_bishops).nonempty() {
                return Bishop;
            } else if (sqbit & opponent_rooks).nonempty() {
                return Rook;
            } else if (sqbit & opponent_queens).nonempty() {
                return Queen;
            } else if (sqbit & opponent_kings).nonempty() {
                return King;
            } else {
                panic!("Attempted to determine opponent piece type at an empty square.");
            }
        };

        let king_moves = KING_TABLE[king_square.idx()];

        if check_multiplicity > 1 && self.stage < MoveGenStage::MultipleCheckKing {
            // If the king is in double+ check, the only legal moves are
            // king moves, so we compute them and return early.
            let king_danger_squares = game.board.attacked(!game.to_move, true);

            for to in king_moves & empty_squares & !king_danger_squares {
                self.list.add(Move::new_quiet(king_square, to, QUIET_FLAG, King));
            }

            for to in king_moves & opponent_pieces & !king_danger_squares {
                self.list.add(Move::new_capture(king_square, to, CAPTURE_FLAG,
                                                King, opp_ptype_at(to)));
            }

            self.stage = MoveGenStage::Finished;
            return self.list.len() > 0;
        }

        let mut capture_mask = Bitboard::new(u64::max_value());
        let mut quiet_mask = Bitboard::new(u64::max_value());

        if check_multiplicity == 1 {
            capture_mask = king_attackers;

            let checker_square = king_attackers.bitscan_forward();

            if opp_ptype_at(checker_square).is_slider() {
                quiet_mask = ray_between_squares(king_square, checker_square);
            } else {
                quiet_mask = Bitboard::new(0);
            }
        }

        // finding pins is expensive (or is it? profile.)
        // don't re-do it if resuming move generation.
        if self.stage == MoveGenStage::Begin {
            self.pin_finder.update(friendly_color, &game.board);
        }

        let pinned_diagonally = self.pin_finder.pinned_diagonally();
        let pinned_nondiagonally = self.pin_finder.pinned_nondiagonally();
        let pinned = self.pin_finder.pinned();

        /***********/
        /* KNIGHTS */
        /***********/
        if self.stage < MoveGenStage::Knight {
            for from in game.board.get_pieces(friendly_color, Knight) & !pinned
            {
                let knight_moves = KNIGHT_TABLE[from.idx()];

                for to in knight_moves & empty_squares & quiet_mask {
                    self.list.add(
                        Move::new_quiet(from, to, QUIET_FLAG, Knight)
                    );
                }

                for to in knight_moves & opponent_pieces & capture_mask {
                    self.list.add(
                        Move::new_capture(from, to, CAPTURE_FLAG,
                                          Knight, opp_ptype_at(to))
                    );
                }
            }

            self.stage = MoveGenStage::Knight;
            if break_on_found && self.list.len() > 0 {
                return true;
            }
        } // end knights

        /***********/
        /* BISHOPS */
        /***********/

        let friendly_bishops = game.board.get_pieces(friendly_color, Bishop);

        if !friendly_bishops.empty() && self.stage < MoveGenStage::Bishop {

            // UNPINNED
            for from in friendly_bishops & !pinned
            {
                let bishop_moves = get_bishop_rays(from, occupied_squares);

                for to in bishop_moves & empty_squares & quiet_mask {
                    self.list.add(
                        Move::new_quiet(from, to, QUIET_FLAG, Bishop)
                    );
                }

                for to in bishop_moves & opponent_pieces & capture_mask {
                    self.list.add(
                        Move::new_capture(from, to, CAPTURE_FLAG,
                                          Bishop, opp_ptype_at(to))
                    );
                }
            }

            // PINNED
            for from in friendly_bishops & pinned_diagonally
            {
                let bishop_moves = get_bishop_rays(from, occupied_squares)
                    & self.pin_finder.diagonal_constraint(from);

                for to in bishop_moves & empty_squares & quiet_mask {
                    self.list.add(
                        Move::new_quiet(from, to, QUIET_FLAG, Bishop)
                    );
                }

                for to in bishop_moves & opponent_pieces & capture_mask {
                    self.list.add(
                        Move::new_capture(from, to, CAPTURE_FLAG,
                                          Bishop, opp_ptype_at(to))
                    );
                }
            }

            self.stage = MoveGenStage::Bishop;
            if break_on_found && self.list.len() > 0 {
                return true;
            }
        } // end bishops

        /*********/
        /* ROOKS */
        /*********/

        if self.stage < MoveGenStage::Rook {
            let friendly_rooks = game.board.get_pieces(friendly_color, Rook);

            // unpinned
            for from in friendly_rooks & !pinned
            {
                let rook_moves = get_rook_rays(from, occupied_squares);

                /* quiets */
                for to in rook_moves & empty_squares & quiet_mask {
                    self.list.add(
                        Move::new_quiet(from, to, QUIET_FLAG, Rook)
                    );
                }

                /* captures */
                for to in rook_moves & opponent_pieces & capture_mask {
                    self.list.add(
                        Move::new_capture(from, to, CAPTURE_FLAG,
                                          Rook, opp_ptype_at(to))
                    );
                }
            }

            // pinned
            for from in friendly_rooks & pinned_nondiagonally
            {
                let rook_moves = get_rook_rays(from, occupied_squares)
                                 & self.pin_finder.nondiagonal_constraint(from);

                /* quiets */
                for to in rook_moves & empty_squares & quiet_mask {
                    self.list.add(
                        Move::new_quiet(from, to, QUIET_FLAG, Rook)
                    );
                }

                /* captures */
                for to in rook_moves & opponent_pieces & capture_mask {
                    self.list.add(
                        Move::new_capture(from, to, CAPTURE_FLAG,
                                          Rook, opp_ptype_at(to))
                    );
                }
            }

            self.stage = MoveGenStage::Rook;
            if break_on_found && self.list.len() > 0 {
                return true;
            }
        } // end rooks

        /*********/
        /* QUEEN */
        /*********/

        if self.stage < MoveGenStage::Queen {
            let friendly_queens = game.board.get_pieces(friendly_color, Queen);
            //OPTIMIZE: early exit if friendly_X is empty?

            for from in friendly_queens & !pinned
            {
                let queen_moves = get_queen_rays(from, occupied_squares);

                /* quiets */
                for to in queen_moves & empty_squares & quiet_mask {
                    self.list.add(
                        Move::new_quiet(from, to, QUIET_FLAG, Queen)
                    );
                }

                /* captures */
                for to in queen_moves & opponent_pieces & capture_mask {
                    self.list.add(
                        Move::new_capture(from, to, CAPTURE_FLAG,
                                          Queen, opp_ptype_at(to))
                    );
                }
            }

            let movable_pinned_queens = friendly_queens & pinned & !(pinned_diagonally & pinned_nondiagonally);

            for from in movable_pinned_queens & pinned_diagonally
            {
                let queen_moves = get_queen_rays(from, occupied_squares)
                                  & self.pin_finder.diagonal_constraint(from);

                /* quiets */
                for to in queen_moves & empty_squares & quiet_mask {
                    self.list.add(
                        Move::new_quiet(from, to, QUIET_FLAG, Queen)
                    );
                }

                /* captures */
                for to in queen_moves & opponent_pieces & capture_mask {
                    self.list.add(
                        Move::new_capture(from, to, CAPTURE_FLAG,
                                          Queen, opp_ptype_at(to))
                    );
                }
            }

            for from in movable_pinned_queens & pinned_nondiagonally
            {
                let queen_moves = get_queen_rays(from, occupied_squares)
                                  & self.pin_finder.nondiagonal_constraint(from);

                /* quiets */
                for to in queen_moves & empty_squares & quiet_mask {
                    self.list.add(
                        Move::new_quiet(from, to, QUIET_FLAG, Queen)
                    );
                }

                /* captures */
                for to in queen_moves & opponent_pieces & capture_mask {
                    self.list.add(
                        Move::new_capture(from, to, CAPTURE_FLAG,
                                          Queen, opp_ptype_at(to))
                    );
                }
            }

            self.stage = MoveGenStage::Queen;
            if break_on_found && self.list.len() > 0 {
                return true;
            }
        } // end queens

        let friendly_pawns = game.board.get_pieces(friendly_color, Pawn);
        let delta_pawn_single_push: i32 = if game.to_move == White { -8 } else { 8 };
        let delta_pawn_double_push: i32 = if game.to_move == White { -16 } else { 16 };
        let double_pawn_push_rank = if game.to_move == White { RANK4 } else { RANK5 };
        let promotion_rank = if game.to_move == White { 8 } else { 1 };

        /*********/
        /* PAWNS */
        /*********/

        if self.stage < MoveGenStage::PawnQuiet {

            let advanceable_pawns = friendly_pawns & !pinned_diagonally;

            let advanced_pawns =
                if friendly_color == White {
                    advanceable_pawns.shifted_up() & empty_squares
                } else {
                    advanceable_pawns.shifted_down() & empty_squares
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
                    self.list.add(Move::new_quiet(from, to, KNIGHT_PROMO_FLAG, Pawn));
                    self.list.add(Move::new_quiet(from, to, BISHOP_PROMO_FLAG, Pawn));
                    self.list.add(Move::new_quiet(from, to, ROOK_PROMO_FLAG, Pawn));
                    self.list.add(Move::new_quiet(from, to, QUEEN_PROMO_FLAG, Pawn));
                } else {
                    self.list.add(Move::new_quiet(from, to, QUIET_FLAG, Pawn));
                }
            }

            let double_advanced_pawns =
                if friendly_color == White {
                    advanced_pawns.shifted_up()
                } else {
                    advanced_pawns.shifted_down()
                };

            // double pushes
            for to in double_advanced_pawns & empty_squares & double_pawn_push_rank & quiet_mask {
                let from = Square::new((to.unwrap() as i32 + delta_pawn_double_push) as u32);

                if (from.bitrep() & pinned_nondiagonally).nonempty()
                    && (to.bitrep() & self.pin_finder.nondiagonal_constraint(from)).empty() {
                        continue;
                    }

                self.list.add(Move::new_quiet(from, to, DOUBLE_PAWN_PUSH_FLAG, Pawn));
            }


            self.stage = MoveGenStage::PawnQuiet;
            if break_on_found && self.list.len() > 0 {
                return true;
            }
        }


        //TODO: fix this mess
        if self.stage < MoveGenStage::PawnCapture {
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
                        self.list.add(Move::new_capture(from, to, KNIGHT_PROMO_CAPTURE_FLAG, Pawn, opp_ptype_at(to)));
                        self.list.add(Move::new_capture(from, to, BISHOP_PROMO_CAPTURE_FLAG, Pawn, opp_ptype_at(to)));
                        self.list.add(Move::new_capture(from, to, ROOK_PROMO_CAPTURE_FLAG, Pawn, opp_ptype_at(to)));
                        self.list.add(Move::new_capture(from, to, QUEEN_PROMO_CAPTURE_FLAG, Pawn, opp_ptype_at(to)));
                    } else {
                        self.list.add(Move::new_capture(from, to, CAPTURE_FLAG, Pawn, opp_ptype_at(to)));
                    }
                }


                match game.ep_square {
                    None => {}
                    Some(ep_capture_square) => {
                        let captured_sq = match opponent_color {
                            White => Square::new(ep_capture_square.unwrap() + 8),
                                  Black => Square::new(ep_capture_square.unwrap() - 8)
                        };

                        //CLEANUP
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
                                    self.list.add(Move::new_capture(from, ep_capture_square, EP_CAPTURE_FLAG,
                                                                    Pawn, opp_ptype_at(captured_sq)));
                                }
                            }
                    }
                }
            }


            self.stage = MoveGenStage::PawnCapture;
            if break_on_found && self.list.len() > 0 {
                return true;
            }
        }


        /********/
        /* KING */
        /********/

        let king_danger_squares = game.board.attacked(!game.to_move, true);

        if self.stage < MoveGenStage::KingNonCastle {

            /* quiets */
            for to in king_moves & empty_squares & !king_danger_squares {
                self.list.add(
                    Move::new_quiet(king_square, to, QUIET_FLAG, King)
                );
            }

            /* captures */
            for to in king_moves & opponent_pieces & !king_danger_squares {
                self.list.add(
                    Move::new_capture(king_square, to, CAPTURE_FLAG,
                                      King, opp_ptype_at(to))
                );
            }

            self.stage = MoveGenStage::KingNonCastle;
            if break_on_found && self.list.len() > 0 {
                return true;
            }
        }

        /* castling */
        if self.stage < MoveGenStage::Finished {
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
                            White => self.list.add(Move::new_quiet(king_square, Square::new(1), KING_CASTLE_FLAG, King)),
                            Black => self.list.add(Move::new_quiet(king_square, Square::new(57), KING_CASTLE_FLAG, King)),
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
                            White => self.list.add(Move::new_quiet(king_square, Square::new(5), QUEEN_CASTLE_FLAG, King)),
                            Black => self.list.add(Move::new_quiet(king_square, Square::new(61), QUEEN_CASTLE_FLAG, King)),
                        }
                    }
                }
            }

            self.stage = MoveGenStage::Finished;
        }

        return self.list.len() > 0;
    }

    /*
    pub fn generate_captures(&mut self, game: &Game) {
        use Color::*;
        use PieceType::*;

        // OPTIMIZE: check if any of these can be moved below
        let friendly_color      = game.to_move;
        let opponent_color      = !friendly_color;
        let empty_squares       = game.board.unoccupied();
        let occupied_squares    = game.board.occupied();
        let friendly_pieces     = game.board.occupied_by(friendly_color);
        let opponent_pieces     = game.board.occupied_by(!friendly_color);
        let king_square         = game.board.get_king_square(friendly_color);
        let king_attackers      = game.king_attackers;
        let check_multiplicity  = king_attackers.population();
        let in_check            = check_multiplicity > 0;
        let king_moves          = KING_TABLE[king_square.idx()];

        if check_multiplicity > 1 {
            let king_danger_squares = game.board.attacked(!game.to_move, true);
            for to in king_moves & opponent_pieces & !king_danger_squares {
                self.list.add(Move::new(king_square, to, CAPTURE_FLAG));
            }
            self.stage = MoveGenStage::Finished;
            return;
        }

        let mut capture_mask = Bitboard::new(u64::max_value());
        let mut quiet_mask = Bitboard::new(u64::max_value());

        if check_multiplicity == 1 {
            capture_mask = king_attackers;

            let checker_square = king_attackers.bitscan_forward();

            if game.piece_squares[checker_square.idx()].unwrap().ptype.is_slider() {
                quiet_mask = ray_between_squares(king_square, checker_square);
            } else {
                quiet_mask = Bitboard::new(0);
            }
        }

        // finding pins is expensive, don't re-do it if resuming move generation.
        if self.stage == MoveGenStage::Begin {
            self.pin_finder.update(friendly_color, &game.board);
        }

        let pinned_diagonally = self.pin_finder.pinned_diagonally();
        let pinned_nondiagonally = self.pin_finder.pinned_nondiagonally();
        let pinned = self.pin_finder.pinned();

        /***********/
        /* KNIGHTS */
        /***********/
        if self.stage < MoveGenStage::Knight {
            for from in game.board.get_pieces(friendly_color, Knight) & !pinned
            {
                let knight_moves = KNIGHT_TABLE[from.idx()];

                for to in knight_moves & opponent_pieces & capture_mask {
                    self.list.add(Move::new(from, to, CAPTURE_FLAG));
                }
            }
        }

        /***********/
        /* BISHOPS */
        /***********/

        let friendly_bishops = game.board.get_pieces(friendly_color, Bishop);

        if !friendly_bishops.empty() {
            // UNPINNED
            for from in friendly_bishops & !pinned
            {
                let bishop_moves = get_bishop_rays(from, occupied_squares);

                for to in bishop_moves & opponent_pieces & capture_mask {
                    self.list.add(Move::new(from, to, CAPTURE_FLAG));
                }
            }

            // PINNED
            for from in friendly_bishops & pinned_diagonally
            {
                let bishop_moves = get_bishop_rays(from, occupied_squares)
                    & self.pin_finder.diagonal_constraint(from);

                for to in bishop_moves & opponent_pieces & capture_mask {
                    self.list.add(Move::new(from, to, CAPTURE_FLAG));
                }
            }
        }

        /*********/
        /* ROOKS */
        /*********/

        let friendly_rooks = game.board.get_pieces(friendly_color, Rook);
        if !friendly_rooks.empty() {

            // unpinned
            for from in friendly_rooks & !pinned
            {
                let rook_moves = get_rook_rays(from, occupied_squares);

                /* captures */
                for to in rook_moves & opponent_pieces & capture_mask {
                    self.list.add(Move::new(from, to, CAPTURE_FLAG));
                }
            }

            // pinned
            for from in friendly_rooks & pinned_nondiagonally
            {
                let rook_moves = get_rook_rays(from, occupied_squares)
                                 & self.pin_finder.nondiagonal_constraint(from);

                /* captures */
                for to in rook_moves & opponent_pieces & capture_mask {
                    self.list.add(Move::new(from, to, CAPTURE_FLAG));
                }
            }
        }

        /*********/
        /* QUEEN */
        /*********/
        let friendly_queens = game.board.get_pieces(friendly_color, Queen);
        if !friendly_queens.empty() {
            for from in friendly_queens & !pinned
            {
                let queen_moves = get_queen_rays(from, occupied_squares);

                /* captures */
                for to in queen_moves & opponent_pieces & capture_mask {
                    self.list.add(Move::new(from, to, CAPTURE_FLAG));
                }
            }

            let movable_pinned_queens = friendly_queens & pinned & !(pinned_diagonally & pinned_nondiagonally);

            for from in movable_pinned_queens & pinned_diagonally
            {
                let queen_moves = get_queen_rays(from, occupied_squares)
                                  & self.pin_finder.diagonal_constraint(from);

                /* captures */
                for to in queen_moves & opponent_pieces & capture_mask {
                    self.list.add(Move::new(from, to, CAPTURE_FLAG));
                }
            }

            for from in movable_pinned_queens & pinned_nondiagonally
            {
                let queen_moves = get_queen_rays(from, occupied_squares)
                                  & self.pin_finder.nondiagonal_constraint(from);

                /* captures */
                for to in queen_moves & opponent_pieces & capture_mask {
                    self.list.add(Move::new(from, to, CAPTURE_FLAG));
                }
            }
        }


        let friendly_pawns = game.board.get_pieces(friendly_color, Pawn);
        let promotion_rank = if game.to_move == White { 8 } else { 1 };

        /*********/
        /* PAWNS */
        /*********/

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
                    self.list.add(Move::new(from, to, BISHOP_PROMO_CAPTURE_FLAG));
                    self.list.add(Move::new(from, to, KNIGHT_PROMO_CAPTURE_FLAG));
                    self.list.add(Move::new(from, to, ROOK_PROMO_CAPTURE_FLAG));
                    self.list.add(Move::new(from, to, QUEEN_PROMO_CAPTURE_FLAG));
                } else {
                    self.list.add(Move::new(from, to, CAPTURE_FLAG));
                }
            }

            match game.ep_square {
                None => {}
                Some(ep_capture_square) => {
                    let captured_sq = match opponent_color {
                        White => Square::new(ep_capture_square.unwrap() + 8),
                              Black => Square::new(ep_capture_square.unwrap() - 8)
                    };

                    //CLEANUP
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
                                self.list.add(Move::new(from, ep_capture_square, EP_CAPTURE_FLAG));
                            }
                        }
                }
            }
        }


        /********/
        /* KING */
        /********/
        let king_danger_squares = game.board.attacked(!game.to_move, true);
        if self.stage < MoveGenStage::KingNonCastle {
            for to in king_moves & opponent_pieces & !king_danger_squares {
                self.list.add(Move::new(king_square, to, CAPTURE_FLAG));
            }
        }

        self.stage = MoveGenStage::Finished;
    }
    */
}

//NOTE: highly inefficient, but this will rarely be used.
pub fn move_from_algebraic(game: &Game, move_str: String) -> Option<Move> {
    if move_str.len() !=4 && move_str.len() != 5 {
        return None;
    }

    let from_str = &move_str[..2];
    let to_str = &move_str[2..4];

    let maybe_from_sq = Square::from_algebraic(from_str);
    if !maybe_from_sq.is_some() {
        return None;
    }
    let from_sq = maybe_from_sq.unwrap();

    let maybe_to_sq = Square::from_algebraic(to_str);
    if !maybe_to_sq.is_some() {
        return None;
    }
    let to_sq = maybe_to_sq.unwrap();

    let is_promotion = move_str.len() == 5;

    if !is_promotion {
        for m in next_moves_standalone(game).iter() {
            if m.from() == from_sq && m.to() == to_sq {
                return Some(*m);
            }
        }
    } else {
        let promo_flag = match move_str.chars().nth(4) {
            Some('k') => KNIGHT_PROMO_FLAG,
            Some('K') => KNIGHT_PROMO_FLAG,
            Some('n') => KNIGHT_PROMO_FLAG,
            Some('N') => KNIGHT_PROMO_FLAG,
            Some('b') => BISHOP_PROMO_FLAG,
            Some('B') => BISHOP_PROMO_FLAG,
            Some('r') => ROOK_PROMO_FLAG,
            Some('R') => ROOK_PROMO_FLAG,
            Some('q') => QUEEN_PROMO_FLAG,
            Some('Q') => QUEEN_PROMO_FLAG,
            _ => 0
        };

        if promo_flag == 0 {
            return None;
        }

        for m in next_moves_standalone(game).iter() {
            let move_flag = m.flag() & 0b1011; // don't need to compare capture status
            if m.from() == from_sq && m.to() == to_sq && (move_flag == promo_flag) {
                return Some(*m);
            }
        }
    }

    return None;
}
