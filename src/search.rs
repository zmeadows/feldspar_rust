use core::*;
use game::*;
use movegen::*;
use moves::*;
use tree::*;
use eval::*;

// use std::thread;

fn maxi(tree: &mut GameTree, max_depth: usize, mut alpha: Score, beta: Score) -> Score {
    if tree.depth == max_depth {
        return Score::recompute(&tree.game);
    }

    let next_moves = tree.next_moves();

    for m in next_moves.borrow().list.iter() {
        let game_copy = tree.game.clone();

        tree.make_move(*m);
        let score = mini(tree, max_depth, alpha, beta);
        tree.unmake_move(game_copy);

        if score >= beta {
            return beta;   // fail hard beta-cutoff
        }

        if score > alpha {
            alpha = score; // alpha acts like max in MiniMax
        }
    }

    return alpha;
}

fn mini(tree: &mut GameTree, max_depth: usize, alpha: Score, mut beta: Score) -> Score {
    if tree.depth == max_depth {
        return Score::recompute(&tree.game);
    }

    let next_moves = tree.next_moves();

    for m in next_moves.borrow().list.iter() {
        let game_copy = tree.game.clone();

        tree.make_move(*m);
        let score = maxi(tree, max_depth, alpha, beta);
        tree.unmake_move(game_copy);

        if score <= alpha {
            return alpha; // fail hard alpha-cutoff
        }

        if score < beta {
            beta = score; // beta acts like min in MiniMax
        }
    }

    return beta;
}

fn alpha_beta_internal(tree: &mut GameTree, max_depth: usize) -> Score {
    match tree.game.to_move {
        Color::White => maxi(tree, max_depth, Score::min(), Score::max()),
        Color::Black => mini(tree, max_depth, Score::min(), Score::max())
    }
}

pub fn alpha_beta(tree: &mut GameTree, max_depth: usize) -> (Move, Score) {
    assert!(tree.depth == 0);

    let mut best_move = Move::null();

    let mut best_score = match tree.game.to_move {
        Color::White => Score::min(),
        Color::Black => Score::max()
    };

    let next_moves = tree.next_moves();

    let game_copy = tree.game.clone();

    for m in next_moves.borrow().list.iter() {
        tree.make_move(*m);

        let score = alpha_beta_internal(tree, max_depth);

        tree.game = game_copy;

        if tree.game.to_move == Color::White && score >= best_score {
            best_score = score;
            best_move = *m;
        } else if tree.game.to_move == Color::Black && score <= best_score {
            best_score = score;
            best_move = *m;
        }

        tree.unmake_move(game_copy);
    }

    assert!(best_move != Move::null());

    return (best_move, best_score);
}

