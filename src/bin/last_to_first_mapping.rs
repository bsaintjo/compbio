extern crate compbio;

use compbio::burrows_wheeler;
use std::io::{self, BufRead};

fn parse_data() -> (String, usize) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let transform = lines.next().unwrap().unwrap();
    let idx = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    (transform, idx)
}

fn main() {
    let (tf, idx) = parse_data();
    let idx_tf: Vec<_> = burrows_wheeler::uniq_count(&tf).collect();
    let idx_char = idx_tf[idx];
    let bwt = burrows_wheeler::BWT { bwt: tf };
    let fst_column = burrows_wheeler::column_hash_map(&bwt.fst_column());
    println!("{}", fst_column[&idx_char.1][idx_char.0]);
}
