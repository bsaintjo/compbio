use itertools::unfold;
use petgraph::dot::Dot;
use petgraph::prelude::*;

#[derive(Default)]
pub struct Trie {
    root: NodeIndex,
    tree: Graph<(), u8>,
}

impl Trie {
    pub fn new() -> Self {
        let mut tree = Graph::new();
        let root = tree.add_node(());
        Trie { root, tree }
    }

    pub fn root(&self) -> &NodeIndex {
        &self.root
    }

    pub fn tree(&self) -> &Graph<(), u8> {
        &self.tree
    }

    pub fn from_patterns<I>(patterns: I) -> Self
    where
        I: Iterator<Item = Vec<u8>>,
    {
        let mut trie = Trie::new();
        for pat in patterns {
            trie.add_pattern(&pat);
        }
        trie
    }

    pub fn add_pattern(&mut self, patt: &[u8]) {
        let mut curr_node = self.root;
        for &ch in patt {
            let some_edge = self
                .tree
                .edge_indices()
                .filter(|&eidx| self.tree.edge_endpoints(eidx).unwrap().0 == curr_node)
                .find(|&eidx| self.tree[eidx] == ch);
            match some_edge {
                Some(eidx) => curr_node = self.tree.edge_endpoints(eidx).unwrap().1,
                None => {
                    let nidx = self.tree.add_node(());
                    let _ = self.tree.add_edge(curr_node, nidx, ch);
                    curr_node = nidx;
                }
            }
        }
    }

    pub fn dot_string(&self) -> String {
        let graph = self.tree.map(|_, _| "", |_, &e| char::from(e));
        format!("{}", Dot::new(&graph))
    }

    fn prefix_trie_matching(&self, text: &[u8]) -> bool {
        let mut curr_node = self.root;
        for x in text {
            if self.tree.edges(curr_node).count() == 0 {
                return true;
            } else if let Some(e) = self.tree.edges(curr_node).find(|e| e.weight() == x) {
                curr_node = e.target()
            } else {
                return false;
            }
        }
        false
    }

    pub fn trie_matching(&self, text: &[u8]) -> Vec<usize> {
        let mut acc = Vec::new();
        let slices = unfold(text, |st| {
            if let Some(new_st) = st.split_first().map(|s| s.1) {
                let old_st = *st;
                *st = new_st;
                Some(old_st)
            } else {
                None
            }
        }).enumerate();
        for (i, s) in slices {
            if self.prefix_trie_matching(s) {
                acc.push(i)
            }
        }
        acc
    }
}
