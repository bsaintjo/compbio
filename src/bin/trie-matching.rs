extern crate compbio;
extern crate itertools;

use std::io::{self, BufRead};

use itertools::Itertools;

struct Dataset {
    text: Vec<u8>,
    compbio: Vec<Vec<u8>>,
}

fn parse_text_patts() -> Dataset {
    let stdin = io::stdin();
    let mut stdin = stdin.lock().split(b'\n');
    let text = stdin.next().unwrap().unwrap();
    let compbio = stdin.map(|s| s.unwrap()).collect();
    Dataset { text, compbio }
}

fn main() {
    let dataset = parse_text_patts();
    let trie = compbio::trie::Trie::from_patterns(dataset.compbio.into_iter());
    let answer = trie.trie_matching(&dataset.text);
    println!("{}", answer.iter().join(" "));
}
