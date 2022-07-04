use compbio::orf;
use compbio::utils;
use seq_io::fasta::Reader;
use std::io;

pub fn main() {
    env_logger::init();

    let mut reader = Reader::new(io::stdin());
    for rec in reader.records() {
        let rec = rec.unwrap();
        let orfs = orf::find_orfs2(&rec.seq);
        for orf in orfs.iter() {
            println!("{}", utils::u8_to_string(orf));
        }
    }
}
