use compbio::edit;
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
    let tbm = edit::wagner_fischer(target, query);
    let dist = edit::distance(target, query, &tbm);
    println!("{}", dist);
}
