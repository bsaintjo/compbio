extern crate compbio;

use compbio::suffix_tree;
use std::io::{self, BufRead};

fn parse_strings() -> (String, String) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut fst = lines.next().unwrap().unwrap();
    let mut snd = lines.next().unwrap().unwrap();
    fst += "$0";
    snd += "$1";
    eprintln!("fst {}", fst);
    eprintln!("snd {}", snd);
    (fst, snd)
}

fn main() {
    let (fst, snd) = parse_strings();
    let lss = suffix_tree::SuffixTree::longest_shared_substring(&fst, &snd);
    println!("{}", lss);
}
