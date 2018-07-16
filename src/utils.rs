use petgraph::dot::Dot;

use suffix_tree;

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
