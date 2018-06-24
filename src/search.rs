use core::*;
use game::*;
use movegen::*;
use moves::*;
use tree::*;
use eval::*;
use zobrist::*;

pub fn negamax(tree: &mut SearchTree, depth_left: usize, mut alpha: Score, beta: Score) -> (Score, Move) {
    if depth_left == 0 || tree.focus().outcome.is_some() {
        let s = Score::recompute(&tree.focus(), tree.search_depth());
        return match tree.focus().to_move {
            Color::White => return (s, tree.last_move()),
            Color::Black => return (s.flipped(), tree.last_move())
        }
    }

    let mut best_move = Move::null();
    let mut best_value = Score::min();
    let next_moves = tree.next_moves();

    for m in next_moves.borrow().iter() {
        let game_copy = *tree.focus();

        tree.make_move(*m);
        let (s1,mb) = negamax(tree, depth_left - 1, beta.flipped(), alpha.flipped());
        tree.unmake_move(game_copy);
        let s2 = s1.flipped();
        if (s2 > best_value) {
            best_move = *m;
            best_value = s2;
        }

        if s2 > alpha {
            alpha = s2;
        }

        if alpha >= beta {
            break;
        }
    }

    return (best_value, best_move);
}
