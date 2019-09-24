extern crate compbio;
extern crate itertools;

use compbio::suffix_array;
use itertools::Itertools;
use std::io::{self, BufRead};

fn parse_text() -> (String, Vec<String>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let text = lines.next().unwrap().unwrap() + "$";
    let compbio = lines.map(|xs| xs.unwrap()).collect();
    (text, compbio)
}

fn main() {
    let (text, compbio) = parse_text();
    let sarray = suffix_array::SuffixArray::new(text);
    let mut idxs: Vec<usize> = compbio
        .into_iter()
        .flat_map(|pattern| {
            let (left, right) = sarray.pattern_match(&pattern).unwrap();
            sarray.index_array()[left..=right].iter().cloned()
        })
        .collect();
    idxs.sort();
    println!("{}", idxs.iter().join(" "));
}
