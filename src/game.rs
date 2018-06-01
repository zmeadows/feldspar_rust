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
}
