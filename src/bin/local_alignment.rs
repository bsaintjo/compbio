use compbio::local;
use compbio::utils;
use env_logger;
use seq_io::fasta::Reader;
use std::io;

fn main() {
    env_logger::init();
    let mut reader = Reader::new(io::stdin());
    let seqs = reader
        .records()
        .map(|rec| rec.unwrap().seq)
        .collect::<Vec<Vec<u8>>>();
    let fst = &seqs[0];
    let snd = &seqs[1];
    let builder = local::Builder::new(&fst, &snd, 5);
    let alignment = builder.align();
    println!("Maximum alignment score: {}", alignment.max_score);
    println!(
        "fst aligned sequence: {}",
        utils::u8_to_string(&alignment.fst_aligned)
    );
    println!(
        "snd aligned sequence: {}",
        utils::u8_to_string(&alignment.snd_aligned)
    );
}
