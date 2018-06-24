use core::*;
use game::*;
use movegen::*;
use moves::*;
use tree::*;
use eval::*;

fn maxi(tree: &mut SearchTree, max_depth: usize, mut alpha: Score, beta: Score) -> (Score, Move) {
    if tree.focus().outcome.is_some() {
        return (Score::recompute(&tree.focus(), tree.search_depth()), Move::null());
    }

    if tree.search_depth() == max_depth {
        let last_move = tree.last_move();
        return (Score::recompute(&tree.focus(), tree.search_depth()), last_move);
        // if !tree.in_quiescence && last_move.is_capture() {
        //     tree.in_quiescence = true;
        //     let qdepth = tree.search_depth() + 15;
        //     let (quiescence_score, mm) = maxi(tree, qdepth, alpha, beta);
        //     tree.in_quiescence = false;
        //     debug_assert!(last_move == tree.last_move());
        //     return (quiescence_score, last_move);
        // } else {
        //     return (Score::recompute(&tree.focus(), tree.search_depth()), last_move);
        // }
    }

    let mut best_move = Move::null();

    let next_moves = tree.next_moves();

    for m in next_moves.borrow().iter() {
        let game_copy = tree.focus().clone();

        tree.make_move(*m);
        let (score, _) = mini(tree, max_depth, alpha, beta);
        tree.unmake_move(&game_copy);

        if score >= beta {
            return (beta, *m);   // fail hard beta-cutoff
        }

        if score > alpha {
            best_move = *m;
            alpha = score; // alpha acts like max in MiniMax
        }
    }

    return (alpha, best_move);
}

fn mini(tree: &mut SearchTree, max_depth: usize, alpha: Score, mut beta: Score) -> (Score, Move) {
    if tree.focus().outcome.is_some() {
        let score = Score::recompute(&tree.focus(), tree.search_depth());
        return (score, Move::null());
    }

    if tree.search_depth() == max_depth {
        let last_move = tree.last_move();
        return (Score::recompute(&tree.focus(), tree.search_depth()), last_move);
        // if !tree.in_quiescence && last_move.is_capture() {
        //     tree.in_quiescence = true;
        //     let qdepth = tree.search_depth() + 15;
        //     let (quiescence_score, mm) = mini(tree, qdepth, alpha, beta);
        //     tree.in_quiescence = false;
        //     assert!(last_move == tree.last_move());
        //     return (quiescence_score, last_move);
        // } else {
        //     return (Score::recompute(&tree.focus(), tree.search_depth()), last_move);
        // }
    }

    let mut best_move = Move::null();

    let next_moves = tree.next_moves();

    for m in next_moves.borrow().iter() {
        let game_copy = tree.focus().clone();

        tree.make_move(*m);
        let (score, _) = maxi(tree, max_depth, alpha, beta);
        tree.unmake_move(&game_copy);

        if score <= alpha {
            return (alpha, *m); // fail hard alpha-cutoff
        }

        if score < beta {
            best_move = *m;
            beta = score; // beta acts like min in MiniMax
        }
    }

    return (beta, best_move);
}

pub fn alpha_beta(tree: &mut SearchTree, max_depth: usize) -> (Score, Move) {
    match tree.focus().to_move {
        Color::White => maxi(tree, max_depth, Score::min(), Score::max()),
        Color::Black => mini(tree, max_depth, Score::min(), Score::max())
    }
}

// pub fn alpha_beta(tree: &mut SearchTree, max_depth: usize) -> (Move, Score) {
//     assert!(tree.depth == 0);
//
//     let mut best_move = Move::null();
//
//     let mut best_score = match tree.focus().to_move {
//         Color::White => Score::min(),
//         Color::Black => Score::max()
//     };
//
//     let next_moves = tree.next_moves();
//
//     let game_copy = tree.focus().clone();
//
//     for m in next_moves.borrow().list.iter() {
//         tree.make_move(*m);
//
//         let score = alpha_beta_internal(tree, max_depth);
//
//         tree.unmake_move(game);
//
//         if tree.focus().to_move == Color::White && score >= best_score {
//             best_score = score;
//             best_move = *m;
//         } else if tree.focus().to_move == Color::Black && score <= best_score {
//             best_score = score;
//             best_move = *m;
//         }
//     }
//
//     assert!(best_move != Move::null());
//
//     return (best_move, best_score);
// }
//
