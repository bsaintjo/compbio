use compbio::lcs_splicing_tb;
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
    let tbm = lcs_splicing_tb::longest_common_subsequence(target, query);
    let ssmotif = lcs_splicing_tb::trace(target, query, &tbm);
    println!("{}", ssmotif.first().unwrap());
}
