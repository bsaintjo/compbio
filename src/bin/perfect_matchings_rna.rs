use std::{error::Error, io::stdin};

use bio::io::fasta;
use compbio::perfect_matchings_rna::count_perfect_matchings;


fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = fasta::Reader::new(stdin()).records();
    let rec = reader.next().unwrap()?;
    let seq = rec.seq();
    let count = count_perfect_matchings(seq);
    println!("{count}");
    Ok(())
}
