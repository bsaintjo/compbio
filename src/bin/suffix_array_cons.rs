extern crate compbio;
extern crate itertools;

use compbio::suffix_array;
use itertools::Itertools;
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
    let sarray = suffix_array::SuffixArray::new(text);
    println!("{}", sarray.index_array().iter().join(", "));
}
