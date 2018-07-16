extern crate compbio;

use compbio::suffix_tree;
use compbio::utils;
use std::io::{self, Read};

fn parse_text() -> String {
    let mut text = String::new();
    let stdin = io::stdin();
    let _ = stdin.lock().read_to_string(&mut text);
    let _ = text.pop();
    text
}

fn main() {
    let text = parse_text();
    let tree = suffix_tree::SuffixTree::from_suffixes(&text);
    println!("{}", utils::suffix_tree_to_dot(&tree));
}
