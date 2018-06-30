use std::io::stdin;
use std::io::BufRead;
use std::str::SplitWhitespace;

use game::*;
use movegen::*;
use moves::*;
use zobrist::*;

pub trait UCIEngine {
    fn name(&self) -> &'static str;
    fn author(&self) -> &'static str;
    fn init(&mut self) -> () {}
    fn reset(&mut self) -> () {}
    fn replace_game(&mut self, new_game: Game, history: Vec<Hash>);
    fn find_best_move(&mut self, wtime: u32, btime: u32, winc: u32, binc: u32) -> ();
    // fn infinite_search(&mut self) -> ();

    //TODO: move to UCIEngine trait default implementation
    fn update_position<'a>(&mut self, args: &mut SplitWhitespace<'a>) {
        let mut g = Game::empty_position();

        match args.next() {
            Some("startpos") => g = Game::starting_position(),
            Some("fen") => {
                g = Game::from_fen(args).unwrap();
            }
            _ => {
                eprintln!("error! invalid position string passed!");
                return;
            }
        }

        match args.next() {
            Some("moves") => {},
            _ => {
                self.replace_game(g, Vec::new());
                return
            }
        }

        let mut history = Vec::new();
        loop {
            if let Some(move_str) = args.next() {
                let m = move_from_algebraic(&g, move_str.to_string()).unwrap();
                g.make_move(m);
                history.push(g.hash);
            } else {
                break;
            }
        }

        eprintln!("FEN re-created by feldspar: {}", g.to_fen());

        self.replace_game(g, history);
    }

    fn parse_go_cmd<'a>(&mut self, args: &mut SplitWhitespace<'a>) {

        let mut wtime = 0;
        let mut btime = 0;
        let mut winc = 0;
        let mut binc = 0;

        loop {
            match args.next() {
                Some("wtime") => wtime = args.next().unwrap().parse().unwrap(),
                Some("btime") => btime = args.next().unwrap().parse().unwrap(),
                Some("winc") => winc = args.next().unwrap().parse().unwrap(),
                Some("binc") => binc = args.next().unwrap().parse().unwrap(),
                Some(_) => break,
                None => break
            }
        }

        eprintln!("TIMES: {} {} {} {}", wtime, btime, winc, binc);

        self.find_best_move(wtime, btime, winc, binc);

    }

    fn run(&mut self) -> () {
        let stdin = stdin();
        for line in stdin.lock().lines() {
            eprintln!("line before received from gui/server: {:?}", line);
            let line = line.unwrap_or("".into());
            eprintln!("line received from gui/server: {}", line);

            let mut params = line.split_whitespace();

            if let Some(first_word) = params.next() {

                match first_word {

                    "uci" => {
                        println!("id name {}", self.name());
                        println!("id author {}", self.author());
                    }

                    "setoption" => println!("{}", line),
                    "isready"    => println!("readyok"),
                    "ucinewgame" => self.reset(),
                    "position"   => self.update_position(&mut params),
                    "quit"       => return,
                    "go"         => self.parse_go_cmd(&mut params),
                    _ => println!("Un-used command from GUI/server: {}", first_word)
                }
            }
        }
    }
}
