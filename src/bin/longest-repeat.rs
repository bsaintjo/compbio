extern crate compbio;

use std::io::{self, Read};

use compbio::suffix_tree;

fn parse_string() -> String {
    let mut text = String::new();
    let stdin = io::stdin();
    let _ = stdin.lock().read_to_string(&mut text);
    let _ = text.pop();
    text.push('$');
    text
}

fn main() {
    let text = parse_string();
    let stree = suffix_tree::SuffixTree::from_suffixes(&text);
    println!("{}", stree.longest_common_repeat())
}
