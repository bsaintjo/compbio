use compbio::glob;
use compbio::utils;
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
    let mut alignment = glob::GlobalAlignment::new(fst.to_owned(), snd.to_owned());
    alignment.align();
    println!(
        "Maximum alignment score: {}",
        alignment.maximum_alignment_score()
    );
    let (fst_aligned, snd_aligned) = alignment.traceback();
    println!(
        "fst aligned sequence: {}",
        utils::u8_to_string(&fst_aligned)
    );
    println!(
        "snd aligned sequence: {}",
        utils::u8_to_string(&snd_aligned)
    );
}
