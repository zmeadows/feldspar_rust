use std::io::stdin;
use std::io::BufRead;
use std::str::SplitWhitespace;

use game::*;
use movegen::*;

pub trait UCIEngine {
    fn name(&self) -> &'static str;
    fn author(&self) -> &'static str;
    fn init(&mut self) -> () {}
    fn reset(&mut self) -> () {}
    fn find_best_move(&mut self) -> ();
    fn replace_game(&mut self, game: Game);
    //fn find_best_move(&mut self, wtime: usize, btime: usize, winc: usize, binc: usize, movestogo: usize) -> ();
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
            _ => return
        }

        loop {
            if let Some(move_str) = args.next() {
                let m = move_from_algebraic(&g, move_str.to_string()).unwrap();
                g.make_move(m);
            } else {
                break;
            }
        }

        eprintln!("FEN: {}", g.to_fen());

        self.replace_game(g);
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
                    "go"         => self.find_best_move(),
                    _ => println!("Un-used command from GUI/server: {}", first_word)
                }
            }
        }
    }
}
