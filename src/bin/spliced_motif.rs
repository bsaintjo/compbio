extern crate compbio;
extern crate itertools;
extern crate seq_io;

use compbio::splicing;
use itertools::Itertools;
use seq_io::fasta::Reader;
use std::io;

fn main() {
    let mut reader = Reader::new(io::stdin());
    let seqs = reader
        .records()
        .map(|rec| rec.unwrap().seq)
        .collect::<Vec<Vec<u8>>>();

    let target = &seqs[0];
    let query = &seqs[1];
    let smotif = splicing::spliced_motif(target, query);
    println!("{}", smotif.iter().join(" "));
}
