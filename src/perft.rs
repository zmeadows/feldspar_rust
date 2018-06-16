#![allow(dead_code)]

use bitboard::*;
use board::*;
use core::*;
use game::*;
use movegen::*;
use moves::*;
use tables::*;

use std::collections::HashMap;
use std::cell::RefCell;
use std::thread;
use std::ops::Add;
use std::os;
use std::process::Command;
use time::PreciseTime;

use prettytable::Table;
use prettytable::cell::Cell;
use prettytable::row::Row;

const QPERFT_PATH: &'static str = "/Users/zac/Code/qperft/qperft";
const MAX_PERFT_DEPTH: usize = 20;

#[derive(Clone)]
struct PerftContext {
    move_gen: MoveGen,
    game: Game,
    result: PerftResult
}

#[derive(Clone)]
struct PerftResult {
    pub node_count  : [usize; MAX_PERFT_DEPTH],
    pub captures    : [usize; MAX_PERFT_DEPTH],
    pub ep_captures : [usize; MAX_PERFT_DEPTH],
    pub castles     : [usize; MAX_PERFT_DEPTH],
    pub promotions  : [usize; MAX_PERFT_DEPTH],
    pub checks      : [usize; MAX_PERFT_DEPTH],
    pub check_mates : [usize; MAX_PERFT_DEPTH]
}

impl PerftResult {
    fn new() -> PerftResult {
        PerftResult {
            node_count  : [0; MAX_PERFT_DEPTH],
            captures    : [0; MAX_PERFT_DEPTH],
            ep_captures : [0; MAX_PERFT_DEPTH],
            castles     : [0; MAX_PERFT_DEPTH],
            promotions  : [0; MAX_PERFT_DEPTH],
            checks      : [0; MAX_PERFT_DEPTH],
            check_mates : [0; MAX_PERFT_DEPTH]
        }
    }
}

impl Add for PerftResult {
    type Output = PerftResult;

    fn add(self, other: PerftResult) -> PerftResult {
        let mut result = PerftResult::new();

        for i in 0 .. MAX_PERFT_DEPTH {
            result.node_count[i]  = self.node_count[i]  + other.node_count[i];
            result.captures[i]    = self.captures[i]    + other.captures[i];
            result.ep_captures[i] = self.ep_captures[i] + other.ep_captures[i];
            result.castles[i]     = self.castles[i]     + other.castles[i];
            result.promotions[i]  = self.promotions[i]  + other.promotions[i];
            result.checks[i]      = self.checks[i]      + other.checks[i];
            result.check_mates[i] = self.check_mates[i] + other.check_mates[i];
        }

        return result;
    }
}

impl PerftContext {
    fn new(perft_game: Game) -> PerftContext {
        PerftContext {
            move_gen: MoveGen::new(),
            game: perft_game,
            result: PerftResult::new()
        }
    }

    fn go(&mut self, max_depth: usize, move_subset: Option<&MoveList>) -> PerftResult
    {
        let move_stack = MoveStack::new();

        match move_subset {
            Some(subset) => {
                let mut old_buffer = move_stack.at_depth(1).borrow_mut();
                old_buffer.clear();
                for m in subset.iter() {
                    old_buffer.add(*m);
                }
            },

            None => {
                self.move_gen.fill_move_buffer(&self.game, move_stack.at_depth(1));
            }
        }

        self.go2(1, max_depth, &move_stack);

        return self.result.clone();
    }

    fn go2(&mut self, current_depth: usize, max_depth: usize, move_stack: &MoveStack) {

        match self.game.outcome(move_stack.at_depth(current_depth).borrow().len()) {
            Some(GameResult::Win(_)) => {
                self.result.check_mates[current_depth - 1] += 1;
            },

            Some(GameResult::Draw) => return,

            None => {}
        }

        if self.game.king_attackers.population() > 0 {
            self.result.checks[current_depth - 1] += 1;
        }

        if current_depth > max_depth {
            return;
        }

        for m in move_stack.at_depth(current_depth).borrow().iter() {
            let game_copy = self.game.clone();
            self.game.make_move(*m);

            self.result.node_count[current_depth] += 1;

            if m.flag() == EP_CAPTURE_FLAG {
                self.result.ep_captures[current_depth] += 1;
            }

            if m.is_capture() {
                self.result.captures[current_depth] += 1;
            }

            if m.flag() == KING_CASTLE_FLAG || m.flag() == QUEEN_CASTLE_FLAG {
                self.result.castles[current_depth] += 1;
            }

            if m.is_promotion() {
                self.result.promotions[current_depth] += 1;
            }

            self.move_gen.fill_move_buffer(&self.game, move_stack.at_depth(current_depth + 1));

            self.go2(current_depth+1, max_depth, move_stack);

            self.game = game_copy;
        }
    }
}

// pub fn perft_divide(game: Game, depth: usize) -> HashMap<String, u32> {
//
//     let mut move_gen = MoveGen::new();
//     let move_buffer = move_gen.move_list(&game);
//
//     let mut results = HashMap::new();
//
//     for m in &move_buffer {
//         let mut game_copy = game.clone();
//         game_copy.make_move(*m);
//         let mut nc = NodeCountContext::new(game_copy);
//         nc.go(1,depth-1);
//         let mut f = m.from().to_algebraic();
//         f.push_str(&m.to().to_algebraic());
//
//         results.insert(f, nc.node_count as u32);
//     }
//
// //     println!(r#"
// //   ___ _____   _____ ___  ___
// //  |   \_ _\ \ / /_ _|   \| __|
// //  | |) | | \ V / | || |) | _|
// //  |___/___| \_/ |___|___/|___|
// //         "#);
// //
// //
// //     game.board.print();
// //     println!("");
// //     println!("DEPTH = {}", depth);
// //     println!("");
// //     for (sq, nc) in &results_vec {
// //         println!("{}: {}", sq, nc);
// //     }
// //
// //     println!("total: {}", total);
//
//     return results;
// }

pub fn perft(game: Game, depth: usize) {
    let num_cpus = num_cpus::get() - 2;

    let mut move_gen = MoveGen::new();
    let move_buffer = alloc_move_buffer();
    move_gen.fill_move_buffer(&game, &move_buffer);

    let mut move_vec = Vec::new();
    for m in move_buffer.borrow().iter() {
        move_vec.push(*m);
    }

    let mut threads = Vec::new();

    let start_time = PreciseTime::now();

    // TODO: divide this up more efficiencly
    // don't let the last thread process only 1 game, for example
    for move_subset in move_vec.chunks(move_vec.len() / num_cpus - 1) {
        let mut move_subset_vec = MoveList::new();
        for m in move_subset {
            move_subset_vec.add(m.clone());
        }

        let game_clone = game.clone();

        threads.push(thread::spawn(move || {
            let mut pc = PerftContext::new(game_clone);
            return pc.go(depth, Some(&move_subset_vec));
        }));
    }

    let mut final_result = PerftResult::new();

    for thread in threads {
        match thread.join() {
            Ok(result) => final_result = final_result + result,
            Err(_) => println!("Failed to join threads for PERFT test.")
        }
    }

    let end_time = PreciseTime::now();

    let mut table = Table::new();
    table.add_row(row![
                  "DEPTH",
                  "NODES",
                  "CAPTURES",
                  "EP CAPTURES",
                  "CASTLES",
                  "PROMOTIONS",
                  "CHECKS",
                  "CHECK-MATES"
    ]);

    for i in 0 .. 20 {
        let c = final_result.node_count[i];
        if c != 0 {

            table.add_row(Row::new(vec![
                                   Cell::new(&i.to_string()),
                                   Cell::new(&final_result.node_count[i].to_string()),
                                   Cell::new(&final_result.captures[i].to_string()),
                                   Cell::new(&final_result.ep_captures[i].to_string()),
                                   Cell::new(&final_result.castles[i].to_string()),
                                   Cell::new(&final_result.promotions[i].to_string()),
                                   Cell::new(&final_result.checks[i].to_string()),
                                   Cell::new(&final_result.check_mates[i].to_string()) ]
                                  )
                         );
        }
    }

    let mut total_nodes: usize = 0;

    for i in 0 .. 20 {
        total_nodes += final_result.node_count[i];
    }

    println!(r#"
 ___ ___ ___ ___ _____
| _ \ __| _ \ __|_   _|
|  _/ _||   / _|  | |
|_| |___|_|_\_|   |_|
        "#);

    game.board.print();
    table.printstd();

    println!("Threads used: {}", num_cpus);
    println!("Total Nodes Processed: {}", total_nodes);
    println!("MNodes/Sec: {:.2}", 1e-6 * total_nodes as f64 / (start_time.to(end_time).num_milliseconds() as f64 / 1000.0));

}

// pub fn qperft_divide(game: Game, depth: usize) -> HashMap<String, u32> {
//     let qperft_command = [
//         &depth.to_string(),
//         &["-", &(depth-1).to_string()].join(""),
//         &game.to_fen()
//     ];
//
//     let qperft_output = Command::new(QPERFT_PATH).args(&qperft_command).output().expect("");
//
//     let qperft_output_str: String = String::from_utf8_lossy(&qperft_output.stdout).to_string();
//
//     let delimit1: String = format!("perft( {}", depth-1);
//     let delimit2: String = format!("perft( {}", depth);
//
//     let mut save = false;
//     let mut relevant_lines = Vec::new();
//
//     for line in qperft_output_str.split("\n") {
//         if (line.contains(&delimit2)) {
//             save = false;
//         }
//
//         if save && line.chars().nth(0).unwrap() == '2' && line.chars().nth(1).unwrap() == '.' {
//             relevant_lines.push(line);
//         }
//
//         if (line.contains(&delimit1)) {
//             save = true;
//         }
//     }
//
//     let mut qperft_results_map = HashMap::new();
//
//     for line in &relevant_lines {
//         let split_line: Vec<&str> = line.split_whitespace().collect();
//         qperft_results_map.insert(split_line[1].to_string(), split_line[4].parse::<u32>().unwrap());
//     }
//
//     return qperft_results_map;
// }
//
// pub fn qperft_debug(game: Game) {
//
//     for depth in 3 .. 8 {
//         println!("depth: {}", depth);
//         let qperft_results = qperft_divide(game.clone(), depth);
//         let feldspar_results = perft_divide(game.clone(), depth);
//         println!("{} {}", qperft_results.len(), feldspar_results.len());
//
//         if (qperft_results.len() != feldspar_results.len()) {
//             game.board.print();
//             println!("{}", game.to_fen());
//
//             for (m,s) in &qperft_results {
//                 match feldspar_results.get(m) {
//                     Some(fs) => {},
//                     None => {
//                         println!("feldspar missing move: {}", m);
//                     }
//                 }
//             }
//
//             for (m,s) in feldspar_results {
//                 match qperft_results.get(&m) {
//                     Some(fs) => {},
//                     None => {
//                         println!("feldspar generated illegal move: {}", m);
//                     }
//                 }
//             }
//
//             return;
//         }
//
//         for (m,s) in qperft_results {
//             match feldspar_results.get(&m) {
//                 Some(fs) =>
//                     if *fs != s {
//                         println!("{} {} {}", m, s, fs);
//
//                         match move_from_algebraic(game.clone(), m) {
//                             Some(mv) => {
//                                 mv.print();
//                                 let mut game_copy = game.clone();
//                                 game_copy.make_move(mv, &mut MoveGen::new());
//                                 println!("{}", game_copy.to_fen());
//                                 game_copy.board.print();
//                                 qperft_debug(game_copy);
//                                 return;
//                             },
//
//                             None => { println!("unexpected weirdness"); }
//                         }
//                     },
//                 None => {}
//             }
//         }
//     }
// }
