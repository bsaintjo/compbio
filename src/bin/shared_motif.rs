extern crate compbio;
extern crate seq_io;

use compbio::suffix_tree::SuffixTree;
use seq_io::fasta::Reader;
use std::io;

fn main() {
    let mut reader = Reader::new(io::stdin());
    let seqs = reader
        .records()
        .map(|rec| String::from_utf8(rec.unwrap().seq).unwrap())
        .collect::<Vec<String>>();

    let klcs = SuffixTree::k_longest_shared_substring(
        &seqs.iter().map(|x| x.as_str()).collect::<Vec<&str>>(),
    );
    println!("{}", klcs);
}
