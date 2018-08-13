use std::cmp::Ordering;
use std::collections::HashSet;

use petgraph::prelude::*;
use petgraph::visit::{EdgeRef, VisitMap, Visitable};

pub struct SuffixTree {
    root: NodeIndex,
    tree: Graph<(), String>,
}

impl SuffixTree {
    pub fn empty() -> Self {
        let mut tree = Graph::new();
        let root = tree.add_node(());
        SuffixTree { root, tree }
    }

    pub fn tree(&self) -> &Graph<(), String> {
        &self.tree
    }

    pub fn from_suffixes(text: &str) -> Self {
        let suffixes = suffixes(text);
        let mut suffix_tree = SuffixTree::empty();
        for suffix in suffixes {
            suffix_tree.add_pattern(suffix);
        }
        suffix_tree
    }

    pub fn longest_common_repeat(&self) -> String {
        self.internal_nodes()
            .into_iter()
            .map(|nidx| self.acc_to_root(nidx))
            .max_by_key(|v| v.len())
            .unwrap()
    }

    pub fn generalized(fst: &str, snd: &str) -> SuffixTree {
        let mut stree = SuffixTree::empty();
        for suffix in suffixes(fst).take(fst.len() - 2) {
            stree.add_pattern(suffix);
        }
        for suffix in suffixes(snd).take(snd.len() - 2) {
            stree.add_pattern(suffix);
        }

        stree
    }

    pub fn longest_shared_substring(fst: &str, snd: &str) -> String {
        let mut stree = SuffixTree::empty();
        for suffix in suffixes(fst) {
            stree.add_pattern(suffix);
        }
        for suffix in suffixes(snd) {
            stree.add_pattern(suffix);
        }

        let shared = stree.contain_zero_and_one();
        shared
            .into_iter()
            .map(|nidx| stree.acc_to_root(nidx))
            .max_by_key(|s| s.len())
            .unwrap()
    }

    pub fn k_longest_shared_substring(strs: &[&str]) -> String {
        let mut stree = SuffixTree::empty();
        for (x, s) in strs.iter().enumerate() {
            let mut s = s.to_string();
            let mut unique = "$".to_string();
            unique.push_str(&x.to_string());
            s.push_str(&unique);
            for suffix in suffixes(&s) {
                eprintln!("{}", suffix);
                stree.add_pattern(&suffix);
            }
        }

        stree.longest_common_repeat()
    }

    pub fn shortest_nonshared_substring(fst: &str, snd: &str) -> String {
        let mut stree = SuffixTree::empty();
        for suffix in suffixes(fst) {
            stree.add_pattern(suffix);
        }
        for suffix in suffixes(snd) {
            stree.add_pattern(suffix);
        }

        stree
            .contain_zero_only()
            .into_iter()
            .map(|(nidx, eidx)| stree.acc_from_nidx_eidx(nidx, eidx))
            .min_by_key(|s| s.len())
            .unwrap()
    }

    fn acc_to_root(&self, mut nidx: NodeIndex) -> String {
        eprintln!("New internal node");
        let mut acc = String::new();
        while self.tree.edges_directed(nidx, Incoming).count() > 0 {
            let eref = self.tree.edges_directed(nidx, Incoming).next().unwrap();
            acc = eref.weight().clone().chars().chain(acc.chars()).collect();
            nidx = eref.source();
            eprintln!("acc so far, {}", &acc);
        }
        acc
    }

    fn acc_from_nidx_eidx(&self, nidx: NodeIndex, eidx: EdgeIndex) -> String {
        let mut acc = self.acc_to_root(nidx);
        acc += &self.tree[eidx][..1];
        acc
    }

    fn internal_nodes(&self) -> Vec<NodeIndex> {
        self.tree
            .node_indices()
            .filter(|&nidx| self.tree.neighbors_directed(nidx, Outgoing).count() > 1)
            .collect::<Vec<NodeIndex>>()
    }

    fn is_branch(&self, nidx: NodeIndex) -> bool {
        self.tree.neighbors_directed(nidx, Outgoing).count() > 0
    }

    /**
     * Returns 2-tuple of HashSet<NodeIndex> that contain at $0 and $1 at the end of them. This
     * function tells you whether that specific node contained the end of the
     * suffix. Useful for SuffixTree built with two patterns. In order to use
     * this for more than one pattern, use `all_hashset`
     */
    fn zero_one_hashset(&self) -> (HashSet<NodeIndex>, HashSet<NodeIndex>) {
        let mut internal: Vec<NodeIndex> = self
            .tree
            .externals(Outgoing)
            .flat_map(|nidx| self.tree.neighbors_directed(nidx, Incoming))
            .collect();
        let mut zero_confirmed: HashSet<NodeIndex> = HashSet::new();
        let mut one_confirmed: HashSet<NodeIndex> = HashSet::new();
        let mut vmap = self.tree.visit_map();
        let mut acc: Vec<NodeIndex> = Vec::new();

        while !internal.is_empty() {
            for node in internal {
                if !vmap.visit(node) {
                    continue;
                }

                for neighbor in self.tree.neighbors_directed(node, Outgoing) {
                    let mut one = false;
                    let mut zero = false;

                    // we can look it up since it should have been confirmed already
                    if self.is_branch(neighbor) {
                        zero = zero || zero_confirmed.contains(&neighbor);
                        one = one || one_confirmed.contains(&neighbor);
                    // is a leaf, so just check the edges
                    } else {
                        let eidx = self.tree.find_edge(node, neighbor).unwrap();
                        zero = zero || self.tree[eidx].contains("$0");
                        one = one || self.tree[eidx].contains("$1");
                    }

                    if one {
                        one_confirmed.insert(node);
                    }

                    if zero {
                        zero_confirmed.insert(node);
                    }
                }

                for parent in self.tree.neighbors_directed(node, Incoming) {
                    if !vmap.is_visited(&parent) {
                        acc.push(parent);
                    }
                }
            }

            internal = acc;
            acc = Vec::new();
        }

        (zero_confirmed, one_confirmed)
    }

    fn contain_zero_and_one(&self) -> HashSet<NodeIndex> {
        let (zero_confirmed, one_confirmed) = self.zero_one_hashset();
        &zero_confirmed & &one_confirmed
    }

    fn contain_zero_only(&self) -> Vec<(NodeIndex, EdgeIndex)> {
        self.tree
            .externals(Outgoing)
            .flat_map(|nidx| self.tree.neighbors_directed(nidx, Incoming))
            .filter_map(|nidx| {
                if let Some(eref) = self
                    .tree
                    .edges_directed(nidx, Outgoing)
                    .find(|eref| eref.weight().contains("$0") && eref.weight().len() > 2)
                {
                    Some((nidx, eref.id()))
                } else {
                    None
                }
            }).collect()
    }

    pub fn add_pattern(&mut self, mut pattern: &str) {
        let mut curr_node = self.root;
        eprintln!("New pattern!!!");
        loop {
            if pattern.is_empty() {
                break;
            }

            match self
                .tree
                .edges(curr_node)
                .inspect(|eref| {
                    eprintln!(
                        "edge: {}, pattern {} = {:?}",
                        eref.weight(),
                        pattern,
                        match_idx(pattern, eref.weight())
                    )
                }).map(|eref| (eref.id(), match_idx(pattern, eref.weight())))
                .max_by_key(|&(_, m)| m)
            {
                // Empty graph, initialize with first pattern
                None | Some((_, Match::None)) => {
                    eprintln!("Initialization");
                    self.add_edge(curr_node, pattern);
                    break;
                }

                // There was an edge with a partial match
                Some((eidx, Match::Partial(sidx))) => {
                    eprintln!("Splitting edge");
                    self.split_edge(eidx, sidx, pattern);
                    break;
                }

                // Add to an already branched node
                Some((eidx, Match::Full)) => {
                    eprintln!("Adding to already branched node");
                    let elen = self.tree[eidx].len();
                    curr_node = self.tree.edge_endpoints(eidx).unwrap().1;
                    pattern = &pattern[elen..];
                }
            }
        }
    }

    fn split_edge(&mut self, eidx: EdgeIndex, split_idx: usize, rest: &str) {
        let (prev, next) = self.tree.edge_endpoints(eidx).unwrap();
        let ew = self.tree.remove_edge(eidx).unwrap();
        let (start, end) = ew.split_at(split_idx);

        let internal_idx = self.tree.add_node(());
        let rest_nidx = self.tree.add_node(());
        self.tree.add_edge(prev, internal_idx, String::from(start));
        self.tree.add_edge(internal_idx, next, String::from(end));
        self.tree
            .add_edge(internal_idx, rest_nidx, String::from(&rest[start.len()..]));
    }

    fn add_edge(&mut self, source: NodeIndex, rest: &str) {
        let target = self.tree.add_node(());
        self.tree.add_edge(source, target, String::from(rest));
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Match {
    Full,
    None,
    Partial(usize),
}

impl PartialOrd for Match {
    fn partial_cmp(&self, other: &Match) -> Option<Ordering> {
        match (*self, *other) {
            (Match::Full, _) | (_, Match::None) => Some(Ordering::Greater),
            (_, Match::Full) | (Match::None, _) => Some(Ordering::Less),
            (Match::Partial(ref x), Match::Partial(ref y)) => x.partial_cmp(y),
        }
    }
}

impl Ord for Match {
    fn cmp(&self, other: &Match) -> Ordering {
        match (*self, *other) {
            (Match::Full, _) | (_, Match::None) => Ordering::Greater,
            (_, Match::Full) | (Match::None, _) => Ordering::Less,
            (Match::Partial(ref x), Match::Partial(ref y)) => x.cmp(y),
        }
    }
}

fn match_idx(xs: &str, ys: &str) -> Match {
    let mut xs = xs.chars().peekable();
    let mut ys = ys.chars().peekable();
    if xs.peek() != ys.peek() {
        return Match::None;
    }

    match xs.zip(ys).position(|(x, y)| x != y) {
        None => Match::Full,
        Some(a) => Match::Partial(a),
    }
}

pub fn suffixes(text: &str) -> Suffixes {
    Suffixes { text, len: 0 }
}

pub struct Suffixes<'a> {
    text: &'a str,
    len: usize,
}

impl<'a> Iterator for Suffixes<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == self.text.len() {
            None
        } else {
            self.len += 1;
            Some(&self.text[self.len - 1..])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_idx() {
        let a = "abcde$";
        let x = "abcd$";
        let y = "abde";
        assert_eq!(Match::Partial(2), match_idx(x, y));
        assert_eq!(Match::Partial(4), match_idx(x, a));
        assert_eq!(Match::Partial(4), match_idx(a, x));
        assert_eq!(Match::Full, match_idx(x, x));
    }

    #[test]
    fn test_suffixes() {
        let text: String = "banana$".into();
        let mut bsuf = suffixes(&text).map(|s| s.into()).collect::<Vec<String>>();
        let mut answer: Vec<String> = vec![
            "banana$".into(),
            "anana$".into(),
            "nana$".into(),
            "ana$".into(),
            "na$".into(),
            "a$".into(),
            "$".into(),
        ];
        bsuf.sort();
        answer.sort();
        assert_eq!(bsuf, answer)
    }

    #[test]
    fn test_subtree_edge_indices() {
        let mut stree = SuffixTree::empty();
        for suffix in suffixes("A$0").take(1) {
            stree.add_pattern(suffix);
        }
        for suffix in suffixes("B$1").take(1) {
            stree.add_pattern(suffix);
        }
        let answer = stree
            .contain_zero_and_one()
            .into_iter()
            .collect::<Vec<NodeIndex>>();
        assert!(answer.len() == 1);
        assert_eq!(answer.into_iter().next().unwrap(), NodeIndex::from(0));
    }

    #[test]
    fn test_add_one_character() {
        let mut xs = String::from("abc");
        let ys = "xyz";
        xs += &ys[..1];
        assert_eq!(xs, String::from("abcx"))
    }

    // #[test]
    // fn test_k_longest_shared_substring() {
    //     let xs = "ABABC";
    //     let ys = "BABCA";
    //     let zs = "ABCBA";
    //     let strs: &[&str] = &[xs, ys, zs];
    //     let answer = SuffixTree::k_longest_shared_substring(strs);
    //     assert_eq!("ABC".to_owned(), answer);
    // }
}
