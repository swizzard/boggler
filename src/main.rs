#[macro_use]
extern crate clap;
extern crate rand;
extern crate boggler;

use boggler::board::Board;
use boggler::solver::BogglerTrie;
use clap::{Arg, App};
use rand::{thread_rng};

fn main() {
    let matches = App::new("Boggler")
                    .about("Generates and optionally solves a random Boggle board")
                    .arg(Arg::with_name("wordlist")
                            .short("w")
                            .long("wordlist")
                            .help("Path to wordlist (default /usr/share/dict/american-english)")
                    .takes_value(true))
                    .arg(Arg::with_name("solve")
                            .short("s")
                            .long("solve")
                            .help("Solve the board?"))
                    .get_matches();
    let mut g = thread_rng();
    let b = &Board::new(&mut g);
    println!("{}", b);
    if value_t!(matches.value_of("solve"), bool).unwrap_or(false) {
        let fname = matches.value_of("wordlist").unwrap_or("/usr/share/dict/american-english");
        if let Some(bt) = BogglerTrie::from_file(fname) {
            println!("{}", bt.solve(b));
        } else {
            println!("Error initializing solver--check wordlist");
        }
    }
}
