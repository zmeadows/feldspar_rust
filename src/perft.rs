use bitboard::*;
use board::*;
use core::*;
use game::*;
use movegen::*;
use moves::*;
use tables::*;

use std::collections::HashMap;
use std::thread;
use std::ops::Add;
use std::os;
use std::process::Command;

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
            result.node_count[i]  = self.node_count[i] + other.node_count[i];
            result.captures[i]    = self.captures[i] + other.captures[i];
            result.ep_captures[i] = self.ep_captures[i] + other.ep_captures[i];
            result.castles[i]     = self.castles[i] + other.castles[i];
            result.promotions[i]  = self.promotions[i] + other.promotions[i];
            result.checks[i]      = self.checks[i] + other.checks[i];
            result.check_mates[i] = self.check_mates[i] + other.check_mates[i];
        }

        return result;
    }
}

#[derive(Clone)]
struct NodeCountContext {
    move_gen: MoveGen,
    game: Game,
    pub node_count: usize
}

impl NodeCountContext {
    fn new(game_to_count: Game) -> NodeCountContext {
        return NodeCountContext {
            move_gen: MoveGen::new(),
            game: game_to_count,
            node_count: 0
        };
    }

    fn go(&mut self, current_depth: usize, max_depth: usize) {
        if (current_depth > max_depth) {
            return;
        }

        let move_buffer = self.move_gen.move_list(&self.game);

        for m in &move_buffer {
            let game_copy = self.game.clone();
            self.game.make_move(*m);

            if (current_depth == max_depth) {
                self.node_count += 1;
            }

            self.go(current_depth+1, max_depth);
            self.game = game_copy;
        }
    }
}

impl PerftContext {
    fn new(perft_game: Game) -> PerftContext {
        return PerftContext {
            move_gen: MoveGen::new(),
            game: perft_game,
            result: PerftResult::new()
        };
    }

    fn go(&mut self, max_depth: usize, move_subset: Option<Vec<Move>>) -> PerftResult {
        self.go2(1, max_depth, move_subset);
        return self.result.clone();
    }

    fn go2(&mut self, current_depth: usize, max_depth: usize, move_subset: Option<Vec<Move>>) {
        let kings = self.game.board.get_pieces(self.game.to_move, PieceType::King);

        // if (kings.empty()) {
        //     println!("no king!");
        //     self.game.board.print();
        //     println!("{}", self.game.to_fen());
        //     return;
        // }

        let king_square         = self.game.board.get_king_square(self.game.to_move);
        let king_attackers      = self.game.board.attackers(king_square, !self.game.to_move);
        let check_multiplicity  = king_attackers.population();

        if check_multiplicity > 0 {
            self.result.checks[current_depth - 1] += 1;
        }

        let move_buffer = match move_subset {
            Some(subset) => subset,
            None => self.move_gen.move_list(&self.game)
        };

        // checkmate, sir/madam.
        if move_buffer.len() == 0 && check_multiplicity > 0 {
            self.result.check_mates[current_depth - 1] += 1;
            return;
        }

        //checkmate, sir/madam.
        if move_buffer.len() == 0 && check_multiplicity == 0 {
            println!("what?");
            self.game.board.print();
            return;
        }

        if (current_depth > max_depth) {
            return;
        }

        for m in &move_buffer {
            let game_copy = self.game.clone();
            self.game.make_move(*m);

            self.result.node_count[current_depth] += 1;

            if (m.flag() == EP_CAPTURE_FLAG) {
                self.result.ep_captures[current_depth] += 1;
            }

            if (m.is_capture()) {
                self.result.captures[current_depth] += 1;
            }

            if (m.flag() == KING_CASTLE_FLAG || m.flag() == QUEEN_CASTLE_FLAG) {
                self.result.castles[current_depth] += 1;
            }

            if m.is_promotion() {
                self.result.promotions[current_depth] += 1;
            }


            self.go2(current_depth+1, max_depth, None);
            self.game = game_copy;
        }
    }
}

pub fn perft_divide(game: Game, depth: usize) -> HashMap<String, u32> {

    let mut move_gen = MoveGen::new();
    let move_buffer = move_gen.move_list(&game);

    let mut results = HashMap::new();

    for m in &move_buffer {
        let mut game_copy = game.clone();
        game_copy.make_move(*m);
        let mut nc = NodeCountContext::new(game_copy);
        nc.go(1,depth-1);
        let mut f = m.from().to_algebraic();
        f.push_str(&m.to().to_algebraic());

        results.insert(f, nc.node_count as u32);
    }

//     println!(r#"
//   ___ _____   _____ ___  ___
//  |   \_ _\ \ / /_ _|   \| __|
//  | |) | | \ V / | || |) | _|
//  |___/___| \_/ |___|___/|___|
//         "#);
// 
// 
//     game.board.print();
//     println!("");
//     println!("DEPTH = {}", depth);
//     println!("");
//     for (sq, nc) in &results_vec {
//         println!("{}: {}", sq, nc);
//     }
// 
//     println!("total: {}", total);

    return results;
}

pub fn perft(game: Game, depth: usize) {

    //TODO: shuffle move buffer
    let mut move_gen = MoveGen::new();
    let move_buffer = move_gen.move_list(&game);


    let num_cpus = 5;

    let mut threads = Vec::new();

    for move_subset in move_buffer.chunks(move_buffer.len() / num_cpus) {

        let mut move_subset_vec = Vec::new();
        for m in move_subset {
            move_subset_vec.push(m.clone());
        }

        let game_clone = game.clone();

        threads.push(thread::spawn(move || {
            let mut pc = PerftContext::new(game_clone);
            return pc.go(depth, Some(move_subset_vec));
        }));

    }


    let mut final_result = PerftResult::new();

    for thread in threads {
        match thread.join() {
            Ok(result) => final_result = final_result + result,
            Err(_) => println!("ERROR IN THREAD JOINING!")
        }
    }

    let mut table = Table::new();
    table.add_row(row!["DEPTH", "NODES", "CAPTURES", "EP CAPTURES", "CASTLES", "PROMOTIONS", "CHECKS", "CHECK-MATES"]);

    for i in 0 .. 20 {
        let c = final_result.node_count[i];
        if (c != 0) {

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

    println!(r#"
 ___ ___ ___ ___ _____
| _ \ __| _ \ __|_   _|
|  _/ _||   / _|  | |
|_| |___|_|_\_|   |_|
        "#);

    game.board.print();
    table.printstd();

}

pub fn qperft_divide(game: Game, depth: usize) -> HashMap<String, u32> {
    let qperft_command = [
        &depth.to_string(),
        &["-", &(depth-1).to_string()].join(""),
        &game.to_fen()
    ];

    let qperft_output = Command::new(QPERFT_PATH).args(&qperft_command).output().expect("");

    let qperft_output_str: String = String::from_utf8_lossy(&qperft_output.stdout).to_string();

    let delimit1: String = format!("perft( {}", depth-1);
    let delimit2: String = format!("perft( {}", depth);

    let mut save = false;
    let mut relevant_lines = Vec::new();

    for line in qperft_output_str.split("\n") {
        if (line.contains(&delimit2)) {
            save = false;
        }

        if save && line.chars().nth(0).unwrap() == '2' && line.chars().nth(1).unwrap() == '.' {
            relevant_lines.push(line);
        }

        if (line.contains(&delimit1)) {
            save = true;
        }
    }

    let mut qperft_results_map = HashMap::new();

    for line in &relevant_lines {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        qperft_results_map.insert(split_line[1].to_string(), split_line[4].parse::<u32>().unwrap());
    }

    return qperft_results_map;
}

pub fn qperft_debug(game: Game) {

    for depth in 3 .. 8 {
        println!("depth: {}", depth);
        let qperft_results = qperft_divide(game.clone(), depth);
        let feldspar_results = perft_divide(game.clone(), depth);
        println!("{} {}", qperft_results.len(), feldspar_results.len());

        if (qperft_results.len() != feldspar_results.len()) {
            game.board.print();
            println!("{}", game.to_fen());

            for (m,s) in &qperft_results {
                match feldspar_results.get(m) {
                    Some(fs) => {},
                    None => {
                        println!("feldspar missing move: {}", m);
                    }
                }
            }

            for (m,s) in feldspar_results {
                match qperft_results.get(&m) {
                    Some(fs) => {},
                    None => {
                        println!("feldspar generated illegal move: {}", m);
                    }
                }
            }

            return;
        }

        for (m,s) in qperft_results {
            match feldspar_results.get(&m) {
                Some(fs) =>
                    if *fs != s {
                        println!("{} {} {}", m, s, fs);

                        match move_from_algebraic(game.clone(), m) {
                            Some(mv) => {
                                mv.print();
                                let mut game_copy = game.clone();
                                game_copy.make_move(mv);
                                println!("{}", game_copy.to_fen());
                                game_copy.board.print();
                                qperft_debug(game_copy);
                                return;
                            },

                            None => { println!("unexpected weirdness"); }
                        }
                    },
                None => {}
            }
        }
    }
}
