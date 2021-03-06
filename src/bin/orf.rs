use std::io;
use compbio::orf;
use compbio::utils;
use env_logger;
use seq_io::fasta::Reader;

pub fn main() {
    env_logger::init();

    let mut reader = Reader::new(io::stdin());
    for rec in reader.records() {
        let rec = rec.unwrap();
        let orfs = orf::find_orfs2(&rec.seq);
        for orf in orfs.iter() {
            println!("{}", utils::u8_to_string(&orf));
        }
    }
}
