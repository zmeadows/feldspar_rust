use core::*;
use game::*;
use movegen::*;
use moves::*;
use tree::*;
use eval::*;
use zobrist::*;

pub struct SearchContext {
    pub tree: SearchTree,
    pub qtree: SearchTree,
    pub table: TranspositionTable,
    pub timer: SearchTimer
}

pub fn negamax(context: &mut SearchContext, depth_left: u8, mut alpha: Score, mut beta: Score) -> (Score, Move) {

    let alpha_orig = alpha;

    let mut best_move_candidate = None;

    match context.table.probe(context.tree.focus().hash) {
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

    if depth_left == 0 || context.tree.focus().outcome.is_some() {
        //OPTIMIZE: this copy is not necessary
        context.qtree.reset_root(*context.tree.focus());
        let (qscore, _) = quiescence(&mut context.qtree, alpha, beta);

        return (qscore, Move::null());
    }

    let mut best_move = Move::null();
    let mut best_value = Score::min();
    let next_moves = context.tree.next_moves(best_move_candidate);

    for m in next_moves.borrow().iter() {
        let game_copy = *context.tree.focus();

        context.tree.make_move(*m);
        let (s1,mb) = negamax(context, depth_left - 1, beta.flipped(), alpha.flipped());
        let s2 = s1.flipped();
        context.tree.unmake_move(game_copy);

        if (s2 > best_value || best_move == Move::null()) {
            best_move = *m;
            best_value = s2;
        }

        if s2 > alpha {
            alpha = s2;
        }

        if alpha >= beta {
            break;
        }

        if context.timer.finished() {
            return (best_value, best_move);
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
            (context.tree.focus().fullmoves % 255) as u8
        );

    context.table.update(context.tree.focus().hash, new_tentry);

    return (best_value, best_move);
}

pub fn quiescence(tree: &mut SearchTree, mut alpha: Score, mut beta: Score) -> (Score, Move) {
    debug_assert!(tree.in_quiescence);

    let stand_pat = Score::recompute_symmetric(&tree.focus(), tree.search_depth());

    if stand_pat >= beta {
        return (beta, Move::null());
    }

    if alpha < stand_pat {
        alpha = stand_pat;
    }

    let next_moves = tree.next_moves(None);

    for m in next_moves.borrow().iter() {
        let game_copy = *tree.focus();

        tree.make_move(*m);
        let (s1,_) = quiescence(tree, beta.flipped(), alpha.flipped());
        tree.unmake_move(game_copy);
        let s2 = s1.flipped();

        if s2 >= beta {
            return (beta, Move::null());
        }

        if s2 > alpha {
            alpha = s2;
        }
    }

    return (alpha, Move::null());
}
