use core::*;
use game::*;
use movegen::*;
use moves::*;
use tree::*;
use eval::*;
use zobrist::*;

//TODO: fix cut/all nodes and make sure you understand them.
pub fn negamax(tree: &mut SearchTree, table: &mut TranspositionTable, depth_left: u8, mut alpha: Score, mut beta: Score) -> (Score, Move) {

    let alpha_orig = alpha;

    let mut best_move_candidate = None;

    match table.probe(tree.focus().hash) {
        None => {},
        Some(tentry) => {
            best_move_candidate = Some(tentry.best_move());
            if tentry.depth() >= depth_left {
                let lookup_score = tentry.score();
                match tentry.node_type() {
                    NodeType::PV => return (lookup_score, Move::null()),
                    NodeType::All => if lookup_score > alpha { alpha = lookup_score }
                    NodeType::Cut => if lookup_score < beta { beta = lookup_score }
                }

                if alpha >= beta {
                    return (lookup_score, Move::null());
                }
            }
        }
    }

    if depth_left == 0 || tree.focus().outcome.is_some() {
        let s = Score::recompute(&tree.focus(), tree.search_depth());

        return match tree.focus().to_move {
            Color::White => return (s, tree.last_move()),
            Color::Black => return (s.flipped(), tree.last_move())
        }
    }

    let mut best_move = Move::null();
    let mut best_value = Score::min();
    let next_moves = tree.next_moves(best_move_candidate);

    for m in next_moves.borrow().iter() {
        let game_copy = *tree.focus();

        tree.make_move(*m);
        let (s1,mb) = negamax(tree, table, depth_left - 1, beta.flipped(), alpha.flipped());
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

    let new_node_type = if best_value <= alpha_orig {
        NodeType::All
    } else if best_value >= beta {
        NodeType::Cut
    } else {
        NodeType::PV
    };

    let new_tentry = EntryData::new(
            best_move,
            best_value,
            depth_left,
            new_node_type,
            (tree.focus().fullmoves % 255) as u8
        );

    table.update(tree.focus().hash, new_tentry);

    return (best_value, best_move);
}
