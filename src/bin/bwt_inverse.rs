extern crate compbio;

use compbio::burrows_wheeler;
use std::io::{self, Read};

fn parse_text() -> String {
    let mut text = String::new();
    let mut stdin = io::stdin();
    let _ = stdin.read_to_string(&mut text);
    text.pop();
    text
}

fn main() {
    let text = parse_text();
    let bwt = burrows_wheeler::BWT { bwt: text };
    println!("{}", bwt.invert());
}
