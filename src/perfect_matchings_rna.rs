use petgraph::Graph;

struct Folder {
    graph: Graph<u8, f64>,
}

impl Folder {
    fn from_rna(rna: &[u8]) -> Self {
        unimplemented!()
    }

    fn n_perfect_matchings(&self) -> usize {
        unimplemented!()
    }
}
