use std::{error::Error, io::stdin};

use bio::io::fasta;
use compbio::consensus_profile::Profile;

fn main() -> Result<(), Box<dyn Error>> {
    let mut profile = Profile::new();
    let reader = fasta::Reader::new(stdin()).records();
    reader.for_each(|rec| {
        let rec = rec.unwrap();
        let seq = rec.seq();
        profile.add_seq(seq);
    });
    println!("{}", profile);
    Ok(())
}
