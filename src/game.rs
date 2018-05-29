use core::*;
use moves::*;
use board::*;
use tables::*;

#[derive(Debug, PartialEq, Clone)]
struct Game {
    board: Board,
    to_move: Color,
    ep_square: Square
}


impl Game {
    pub fn fill_move_buffer(&self, move_buffer: &mut Vec<Move>) {
        move_buffer.clear();

        let empty_squares = self.board.unoccupied();

        use Color::*;
        use PieceType::*;

        if self.to_move == White {

            let black_pieces = self.board.occupied_by(Black);

            /*********/
            /* PAWNS */
            /*********/

            let white_pawns = self.board.get_pieces(White, Pawn);
            let advanced_pawns = white_pawns.shifted_up();

            // single pushes (and promotions)
            for to in advanced_pawns & empty_squares
            {
                let from = Square::new(to.unwrap() - 8);

                if to.unwrap()/8 == 7 {
                    move_buffer.push(Move::new(from, to, BISHOP_PROMO_FLAG));
                    move_buffer.push(Move::new(from, to, KNIGHT_PROMO_FLAG));
                    move_buffer.push(Move::new(from, to, ROOK_PROMO_FLAG));
                    move_buffer.push(Move::new(from, to, QUEEN_PROMO_FLAG));
                } else {
                    move_buffer.push(Move::new(from, to, QUIET_FLAG));
                }
            }

            // double pushes
            for to in advanced_pawns.shifted_up() & empty_squares & RANK4 {
                let from = Square::new(to.unwrap() - 16);
                move_buffer.push(Move::new(from, to, DOUBLE_PAWN_PUSH_FLAG));
            }

            // captures (and capture-promotions)
            for from in white_pawns
            {
                for to in PAWN_ATTACKS[White as usize][from.idx()] & black_pieces
                {
                    if to.unwrap()/8 == 7 {
                        move_buffer.push(Move::new(from, to, BISHOP_PROMO_CAPTURE_FLAG));
                        move_buffer.push(Move::new(from, to, KNIGHT_PROMO_CAPTURE_FLAG));
                        move_buffer.push(Move::new(from, to, ROOK_PROMO_CAPTURE_FLAG));
                        move_buffer.push(Move::new(from, to, QUEEN_PROMO_CAPTURE_FLAG));
                    } else if self.ep_square == to {
                        move_buffer.push(Move::new(from, to, EP_CAPTURE_FLAG));
                    } else {
                        move_buffer.push(Move::new(from, to, CAPTURE_FLAG));
                    }
                }
            }

            /***********/
            /* KNIGHTS */
            /***********/

            for from in self.board.get_pieces(White, Knight)
            {
                let knight_moves = KNIGHT_TABLE[from.idx()];

                /* quiets */
                for to in knight_moves & empty_squares {
                    move_buffer.push(Move::new(from, to, QUIET_FLAG));
                }

                /* captures */
                for to in knight_moves & black_pieces {
                    move_buffer.push(Move::new(from, to, CAPTURE_FLAG));
                }

            }

            /***********/
            /* BISHOPS */
            /***********/

            /* 
            temp_piece = game->board[BISHOPS][WHITE];
            while (temp_piece)
            for from in self.board.get_pieces(White, Bishop)
            {
                temp_move2 = get_bishop_rays(game->board[OCCUPIED][1], from);

                /* quiets */
                temp_move = temp_move2 & free_squares;
                while (temp_move)
                {
                    to = bit_scan_forward(temp_move);
                    game->move_buffer[move_count++] = create_move(from,to,QUIET_FLAG);
                    temp_move &= temp_move - 1;
                }

                /* captures */
                temp_move = temp_move2 & game->board[PIECES][BLACK];
                while (temp_move)
                {
                    to = bit_scan_forward(temp_move);
                    game->move_buffer[move_count++] = create_move(from,to,CAPTURE_FLAG);
                    temp_move &= temp_move - 1;
                }

                temp_piece &= temp_piece - 1;
            }
        */



        }
    }
}
