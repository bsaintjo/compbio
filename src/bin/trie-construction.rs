extern crate compbio;
extern crate petgraph;

use compbio::trie;
use petgraph::visit::EdgeRef;
use std::io::{self, BufRead};

type Seq = Vec<u8>;

fn parse_sequence() -> Vec<Seq> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut seqs = Vec::new();
    while let Some(Ok(seq)) = lines.next() {
        seqs.push(seq.into_bytes())
    }
    seqs
}

fn main() {
    let seqs = parse_sequence();
    let trie = trie::Trie::from_patterns(seqs.into_iter());
    for eref in trie.tree().edge_references() {
        println!(
            "{}->{}:{}",
            eref.source().index(),
            eref.target().index(),
            char::from(*eref.weight())
        )
    }
}
