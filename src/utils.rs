use petgraph::dot::Dot;

use crate::lcs_stree;
use crate::suffix_tree;
use itertools::Itertools;
use std::fs::File;
use std::io::Write;

pub fn u8_to_string(x: &[u8]) -> String {
    String::from_utf8(Vec::from(x)).unwrap()
}

// pub fn suffix_tree_to_string(st: &suffix_tree::SuffixTree) -> String {
//     format!("{:?}", st.tree().map(|_, &x| x, |_, e| e.clone()).unwrap()))
// }

pub fn suffix_tree_to_dot(st: &suffix_tree::SuffixTree) -> String {
    let g = st.tree().map(|_, _| "", |_, e| e);
    format!("{}", Dot::new(&g))
}

pub fn lcs_stree_to_dot(st: &lcs_stree::SuffixTree) -> String {
    let g = st.tree().map(
        |_, n| {
            let mut vec_n: Vec<&usize> = n.iter().collect();
            vec_n.sort();
            vec_n.iter().join(",")
        },
        |_, e| e,
    );
    format!("{}", Dot::new(&g))
}

pub fn write_tree_to_dot(output_file: &str, stree: &lcs_stree::SuffixTree) {
    let dot_fmt = lcs_stree_to_dot(stree);
    let mut file = File::create(output_file).unwrap();
    file.write_all(dot_fmt.as_bytes()).unwrap();
}
