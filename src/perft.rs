use core::*;
use bitboard::*;
use moves::*;
use movegen::*;
use board::*;
use tables::*;
use game::*;

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

struct PerftContext {
    move_gen: MoveGen,
    game: Game,
    pub node_count: [usize;20],
    pub captures: [usize;20],
    pub ep_captures: [usize;20],
    pub castles: [usize;20]
}

fn collect_and_display(perfts: Vec<PerftContext>) {
    return;
}

impl PerftContext {
    fn new(perft_game: &Game) -> PerftContext {
        return PerftContext {
            move_gen: MoveGen::new(),
            game: *perft_game,
            node_count: [0; 20],
            captures: [0; 20],
            ep_captures: [0; 20],
            castles: [0; 20]
        };
    }

    fn go(&mut self, current_depth: usize, max_depth: usize) {
        if (current_depth > max_depth) {
            return;
        }

        let move_buffer = self.move_gen.move_list(&self.game);

        for m in &move_buffer {
            let game_copy = self.game;
            self.game.make_move(*m);

            self.node_count[current_depth] += 1;

            if (m.flag() == EP_CAPTURE_FLAG) {
                self.ep_captures[current_depth] += 1;
            }

            if (m.is_capture()) {
                self.captures[current_depth] += 1;
            }

            if (m.flag() == KING_CASTLE_FLAG || m.flag() == QUEEN_CASTLE_FLAG) {
                self.castles[current_depth] += 1;
            }

            self.go(current_depth+1, max_depth);
            self.game = game_copy;
        }
    }

}

pub fn perft(game: &Game, depth: usize) {
    let mut pc = PerftContext::new(game);
    pc.go(1,depth);

    let mut table = Table::new();

    table.add_row(row!["DEPTH", "NODES", "CAPTURES", "EP CAPTURES", "CASTLES", "PROMOTIONS", "CHECKS", "CHECK-MATES"]);

    for i in 0 .. 20 {
        let c = pc.node_count[i];
        if (c != 0) {

            table.add_row(Row::new(vec![
                                   Cell::new(&i.to_string()),
                                   Cell::new(&pc.node_count[i].to_string()),
                                   Cell::new(&pc.captures[i].to_string()),
                                   Cell::new(&pc.ep_captures[i].to_string()),
                                   Cell::new(&pc.castles[i].to_string()),
                                   Cell::new("0"),
                                   Cell::new("0"),
                                   Cell::new("0")]
                                  )
                         );

        }
    }

    game.board.print();
    table.printstd();
}

// pub fn perft_divide(game: mut Game, depth: usize) {
//     let mut pc = PerftContext::new(game);
//
//     let move_buffer = self.move_gen.move_list(&self.game);
//
//     pc.go(1,depth);
//     for i in 0 .. 20 {
//         let c = pc.node_count[i];
//         if (c != 0) {
//             println!("NODES {}: {}", i, pc.node_count[i]);
//             println!("CAPTURES {}: {}", i, pc.captures[i]);
//             println!("EP CAPTURES {}: {}", i, pc.ep_captures[i]);
//             println!("CASTLES {}: {}", i, pc.castles[i]);
//         }
//     }
// }
