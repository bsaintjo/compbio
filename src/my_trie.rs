pub struct Trie {
    root: NodeIdx,
    nodes: Vec<Node>,
    edges: Vec<Edge>
}

type NodeIdx = usize;
type EdgeIdx = usize;

struct Node {
    edges: Vec<EdgeIdx>,
}

struct Edge {
    data: u8,
    nidx: NodeIdx,
}

impl Trie {
    fn from_patterns<'a, I>(patterns: I) -> Trie
        where I: 'a + Iterator<Item=&'a [u8]>
    {
        for patt in patterns {
        }
        unimplemented!()
    }

    fn add_node(&mut self) -> NodeIdx {
        let node = Node { edges: Vec::new() };
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn add_edge(&mut self, data: u8, from: NodeIdx, to: NodeIdx) -> EdgeIdx {
        let edge = Edge { data, nidx: to };
        self.edges.push(edge);
        let eidx = self.edges.len() - 1;
        self.nodes[from].edges.push(eidx);
        eidx
    }

    fn matching_edge(&self, nidx: usize, x: u8) -> Option<EdgeIdx> {
        for &eidx in &self.nodes[nidx].edges {
            if self.edges[eidx].data == x {
                return Some(eidx);
            }
        }
        None
    }

    fn add_patt(&mut self, patt: &[u8]) {
        let mut curr_node = self.root;
        for &x in patt {
            if let Some(eidx) = self.matching_edge(curr_node, x) {
                curr_node = self.edges[eidx].nidx;
            } else {
                let nidx = self.add_node();
                let _ = self.add_edge(x, curr_node, nidx);
                curr_node = nidx;
            }
        }
    }
}
