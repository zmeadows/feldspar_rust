#![allow(dead_code)]

use bitboard::*;
use board::*;
use core::*;
use game::*;
use movegen::*;
use moves::*;
use tables::*;
use tree::*;

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

struct PerftContext {
    tree: GameTree,
    result: PerftResult
}

#[derive(PartialEq, Clone)]
pub struct PerftResult {
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
        let mut new_result = PerftResult {
            node_count  : [0; MAX_PERFT_DEPTH],
            captures    : [0; MAX_PERFT_DEPTH],
            ep_captures : [0; MAX_PERFT_DEPTH],
            castles     : [0; MAX_PERFT_DEPTH],
            promotions  : [0; MAX_PERFT_DEPTH],
            checks      : [0; MAX_PERFT_DEPTH],
            check_mates : [0; MAX_PERFT_DEPTH]
        };

        new_result.node_count[0] = 1;

        return new_result;
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
            tree: GameTree::new(perft_game),
            result: PerftResult::new()
        }
    }

    fn go(&mut self, max_depth: usize) {

        if self.tree.depth == max_depth {
            return;
        }

        let next_moves = self.tree.next_moves();

        for m in next_moves.borrow().list.iter() {
            let game_copy = self.tree.game.clone();

            self.tree.make_move(*m);

            self.result.node_count[self.tree.depth] += 1;

            if m.flag() == EP_CAPTURE_FLAG {
                self.result.ep_captures[self.tree.depth] += 1;
            }

            if m.is_capture() {
                self.result.captures[self.tree.depth] += 1;
            }

            if m.flag() == KING_CASTLE_FLAG || m.flag() == QUEEN_CASTLE_FLAG {
                self.result.castles[self.tree.depth] += 1;
            }

            if m.is_promotion() {
                self.result.promotions[self.tree.depth] += 1;
            }

            if self.tree.game.king_attackers.population() > 0 {
                self.result.checks[self.tree.depth] += 1;
            }

            match self.tree.game.outcome {
                Some(GameResult::Win(_)) => self.result.check_mates[self.tree.depth] += 1,
                _ => {}
            }

            self.go(max_depth);
            self.tree.unmake_move(game_copy);
        }
    }
}


pub fn perft(game: Game, depth: usize) -> PerftResult {
    // let num_cpus = num_cpus::get() - 2;

    // let mut threads = Vec::new();

    let start_time = PreciseTime::now();

    // for move_subset in move_vec.chunks(move_vec.len() / num_cpus - 1) {
    //     let mut move_subset_vec = MoveList::new();
    //     for m in move_subset {
    //         move_subset_vec.add(m.clone());
    //     }

    //     let game_clone = game.clone();

    //     threads.push(thread::spawn(move || {
    //         let mut pc = PerftContext::new(game_clone);
    //         return pc.go(depth, Some(&move_subset_vec));
    //     }));
    // }

    // let mut final_result = PerftResult::new();

    // for thread in threads {
    //     match thread.join() {
    //         Ok(result) => final_result = final_result + result,
    //         Err(_) => println!("Failed to join threads for PERFT test.")
    //     }
    // }

    let mut pc = PerftContext::new(game.clone());
    pc.go(depth);

    let final_result = &pc.result;
    
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

    // println!("Threads used: {}", num_cpus);
    println!("Total Nodes Processed: {}", total_nodes);
    println!("MNodes/Sec: {:.2}", 1e-6 * total_nodes as f64 / (start_time.to(end_time).num_milliseconds() as f64 / 1000.0));

    return final_result.clone();
}

// pub fn perft_divide(game: Game, depth: usize) -> HashMap<String, u32> {
// 
//     let mut move_gen = MoveGen::new();
//     let move_buffer = move_gen.move_list(&game);
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
//     return results;
// }

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

#[cfg(test)]
mod test {
    use perft::*;

    #[test]
    fn standard_position() {
        let mut correct_result = PerftResult::new();

        correct_result.node_count[1] = 20;
        correct_result.node_count[2] = 400;
        correct_result.node_count[3] = 8902;
        correct_result.node_count[4] = 197281;
        correct_result.node_count[5] = 4865609;
        correct_result.node_count[6] = 119060324;

        correct_result.captures[1] = 0;
        correct_result.captures[2] = 0;
        correct_result.captures[3] = 34;
        correct_result.captures[4] = 1576;
        correct_result.captures[5] = 82719;
        correct_result.captures[6] = 2812008;

        correct_result.ep_captures[1] = 0;
        correct_result.ep_captures[2] = 0;
        correct_result.ep_captures[3] = 0;
        correct_result.ep_captures[4] = 0;
        correct_result.ep_captures[5] = 258;
        correct_result.ep_captures[6] = 5248;

        correct_result.castles[1] = 0;
        correct_result.castles[2] = 0;
        correct_result.castles[3] = 0;
        correct_result.castles[4] = 0;
        correct_result.castles[5] = 0;
        correct_result.castles[6] = 0;

        correct_result.promotions[1] = 0;
        correct_result.promotions[2] = 0;
        correct_result.promotions[3] = 0;
        correct_result.promotions[4] = 0;
        correct_result.promotions[5] = 0;
        correct_result.promotions[6] = 0;

        correct_result.checks[1] = 0;
        correct_result.checks[2] = 0;
        correct_result.checks[3] = 12;
        correct_result.checks[4] = 469;
        correct_result.checks[5] = 27351;
        correct_result.checks[6] = 809099;

        correct_result.check_mates[1] = 0;
        correct_result.check_mates[2] = 0;
        correct_result.check_mates[3] = 0;
        correct_result.check_mates[4] = 8;
        correct_result.check_mates[5] = 347;
        correct_result.check_mates[6] = 10828;

        let g = Game::starting_position();
        let result = perft(g, 6);

        assert!(result == correct_result);
    }

    #[test]
    fn kiwipete() {
        let mut correct_result = PerftResult::new();

        correct_result.node_count[1] = 48;
        correct_result.node_count[2] = 2039;
        correct_result.node_count[3] = 97862;
        correct_result.node_count[4] = 4085603;
        correct_result.node_count[5] = 193690690;

        correct_result.captures[1] = 8;
        correct_result.captures[2] = 351;
        correct_result.captures[3] = 17102;
        correct_result.captures[4] = 757163;
        correct_result.captures[5] = 35043416;

        correct_result.ep_captures[1] = 0;
        correct_result.ep_captures[2] = 1;
        correct_result.ep_captures[3] = 45;
        correct_result.ep_captures[4] = 1929;
        correct_result.ep_captures[5] = 73365;

        correct_result.castles[1] = 2;
        correct_result.castles[2] = 91;
        correct_result.castles[3] = 3162;
        correct_result.castles[4] = 128013;
        correct_result.castles[5] = 4993637;

        correct_result.promotions[1] = 0;
        correct_result.promotions[2] = 0;
        correct_result.promotions[3] = 0;
        correct_result.promotions[4] = 15172;
        correct_result.promotions[5] = 8392;

        correct_result.checks[1] = 0;
        correct_result.checks[2] = 3;
        correct_result.checks[3] = 993;
        correct_result.checks[4] = 25523;
        correct_result.checks[5] = 3309887;

        correct_result.check_mates[1] = 0;
        correct_result.check_mates[2] = 0;
        correct_result.check_mates[3] = 1;
        correct_result.check_mates[4] = 43;
        correct_result.check_mates[5] = 30171;

        let g = Game::from_fen_str("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        let result = perft(g, 5);

        assert!(result == correct_result);

    }
}
