extern crate compbio;
extern crate env_logger;
extern crate seq_io;

use compbio::lcs_stree::SuffixTree;
use seq_io::fasta::Reader;
use std::io;

fn main() {
    env_logger::init();
    let mut reader = Reader::new(io::stdin());
    let seqs = reader
        .records()
        .map(|rec| String::from_utf8(rec.unwrap().seq).unwrap())
        .collect::<Vec<String>>();

    let stree = SuffixTree::from_strings(&seqs);
    println!("{}", stree.longest_shared_substring2(seqs.len()));
}
