use std::cmp::Ordering;
use std::collections::HashSet;

use petgraph::prelude::*;
use petgraph::visit::EdgeRef;

pub struct SuffixTree {
    root: NodeIndex,
    tree: Graph<HashSet<usize>, String>,
}

impl SuffixTree {
    pub fn empty() -> Self {
        let mut tree = Graph::new();
        let root = tree.add_node(HashSet::new());
        SuffixTree { root, tree }
    }

    pub fn tree(&self) -> &Graph<HashSet<usize>, String> {
        &self.tree
    }

    fn add_node(&mut self) -> NodeIndex {
        self.tree.add_node(HashSet::new())
    }

    fn add_node_with_idents(&mut self, hset: HashSet<usize>) -> NodeIndex {
        self.tree.add_node(hset)
    }

    /**
     * Given a collection of strings, build the corresponding generalized suffix tree.
     */
    pub fn from_strings(patts: &[String]) -> Self {
        let mut stree = SuffixTree::empty();
        for (ident, patt) in patts.iter().enumerate() {
            let mut unique = "$".to_owned();
            unique.push_str(&ident.to_string());
            let mut patt = patt.clone();
            patt.push_str(&unique);
            for suffix in suffixes(&patt) {
                stree.add_pattern(ident, suffix)
            }
        }
        stree
    }

    pub fn longest_common_repeat(&self) -> String {
        self.internal_nodes()
            .into_iter()
            .map(|nidx| self.acc_to_root(nidx))
            .max_by_key(|v| v.len())
            .unwrap()
    }

    pub fn longest_shared_substring(&self, n: usize) -> String {
        let mut curr_node = self.root;
        let mut acc = String::new();

        while self.tree[curr_node].len() == n {
            let new_node: Vec<NodeIndex> = self
                .tree
                .neighbors(curr_node)
                .filter(|&nidx| self.tree[nidx].len() == n)
                .collect();
            if new_node.is_empty() {
                break;
            } else if new_node.len() > 1 {
                error!("Error in suffix construction");
            } else {
                let new_node = new_node[0];
                let eidx = self.tree.find_edge(curr_node, new_node).unwrap();
                acc.push_str(&self.tree[eidx]);
                curr_node = new_node;
            }
        }
        acc
    }

    pub fn longest_shared_substring2(&self, n: usize) -> String {
        self.tree()
            .node_indices()
            .filter(|&nidx| self.tree[nidx].len() == n)
            .map(|nidx| self.acc_to_root(nidx).split('$').next().unwrap().to_owned())
            .max_by_key(|s| s.len())
            .unwrap()
    }

    pub fn shortest_nonshared_substring(fst: &str, snd: &str) -> String {
        let mut stree = SuffixTree::empty();
        for suffix in suffixes(fst) {
            stree.add_pattern(0, suffix);
        }
        for suffix in suffixes(snd) {
            stree.add_pattern(1, suffix);
        }

        stree
            .contain_zero_only()
            .into_iter()
            .map(|(nidx, eidx)| stree.acc_from_nidx_eidx(nidx, eidx))
            .min_by_key(|s| s.len())
            .unwrap()
    }

    fn acc_to_root(&self, mut nidx: NodeIndex) -> String {
        debug!("New internal node");
        let mut acc = String::new();
        while self.tree.edges_directed(nidx, Incoming).count() > 0 {
            let eref = self.tree.edges_directed(nidx, Incoming).next().unwrap();
            acc = eref.weight().clone().chars().chain(acc.chars()).collect();
            nidx = eref.source();
            debug!("acc so far, {}", &acc);
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
            })
            .collect()
    }

    /** Adds a new string pattern to the suffix tree, adds new nodes and edges as necessary to
     * maintian the suffix tree structure.
     */
    pub fn add_pattern(&mut self, ident: usize, mut pattern: &str) {
        let mut curr_node = self.root;
        debug!("Adding pattern: {}", pattern);
        loop {
            if pattern.is_empty() {
                break;
            }

            match self
                .tree
                .edges(curr_node)
                .inspect(|eref| {
                    debug!(
                        "edge: {}, pattern {} = {:?}",
                        eref.weight(),
                        pattern,
                        match_idx(pattern, eref.weight())
                    )
                })
                .map(|eref| (eref.id(), match_idx(pattern, eref.weight())))
                .max_by_key(|&(_, m)| m)
            {
                // Empty graph, initialize with first pattern
                None | Some((_, Match::None)) => {
                    debug!("Initialization");
                    self.add_edge(curr_node, pattern, ident);
                    break;
                }

                // There was an edge with a partial match
                Some((eidx, Match::Partial(sidx))) => {
                    debug!("Splitting edge");
                    self.split_edge(eidx, sidx, pattern, ident);
                    break;
                }

                // Add to an already branched node
                // The edge string completely matched with the prefix of the pattern
                // So we move the curr_node to node that the full match points to
                // and restart
                Some((eidx, Match::Full)) => {
                    debug!("Adding to already branched node");
                    let elen = self.tree[eidx].len();
                    curr_node = self.tree.edge_endpoints(eidx).unwrap().1;
                    self.tree[curr_node].insert(ident);
                    pattern = &pattern[elen..];
                }
            }
        }
    }

    fn split_edge(&mut self, eidx: EdgeIndex, split_idx: usize, rest: &str, ident: usize) {
        let (prev, next) = self.tree.edge_endpoints(eidx).unwrap();
        let ew = self.tree.remove_edge(eidx).unwrap();
        let (start, end) = ew.split_at(split_idx);

        let mut next_hset = self.tree[next].clone();
        next_hset.insert(ident);
        let internal_idx = self.add_node_with_idents(next_hset.clone());
        let rest_nidx = self.add_node_with_idents(next_hset);
        self.tree.add_edge(prev, internal_idx, String::from(start));
        self.tree.add_edge(internal_idx, next, String::from(end));
        self.tree
            .add_edge(internal_idx, rest_nidx, String::from(&rest[start.len()..]));
    }

    fn add_edge(&mut self, source: NodeIndex, rest: &str, ident: usize) {
        let target = self.add_node();
        self.tree[target].insert(ident);
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
    use petgraph::dot::Dot;
    use utils;

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
    fn test_add_one_character() {
        let mut xs = String::from("abc");
        let ys = "xyz";
        xs += &ys[..1];
        assert_eq!(xs, String::from("abcx"))
    }

    #[test]
    fn test_k_longest_shared_substring() {
        let xs = "ABABC".to_owned();
        let ys = "BABCA".to_owned();
        let zs = "ABCBA".to_owned();
        let strs: &[String] = &[xs, ys, zs];
        let answer = SuffixTree::from_strings(strs);
        utils::write_tree_to_dot("test.dot", &answer);
        assert_eq!("ABC".to_owned(), answer.longest_shared_substring2(3));
    }
}
