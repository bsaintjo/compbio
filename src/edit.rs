use itertools::Itertools;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum AlignOp {
    Insertion,
    Deletion,
    Match,
    Nothing,
}

#[derive(Clone)]
pub struct Cell {
    dist: usize,
    op: AlignOp,
}

impl Cell {
    fn new(dist: usize) -> Self {
        Cell {
            dist,
            op: AlignOp::Nothing,
        }
    }

    fn matches(dist: usize) -> Self {
        Cell {
            dist,
            op: AlignOp::Match,
        }
    }

    fn insertion(dist: usize) -> Self {
        Cell {
            dist,
            op: AlignOp::Insertion,
        }
    }

    fn deletion(dist: usize) -> Self {
        Cell {
            dist,
            op: AlignOp::Deletion,
        }
    }

    fn dist(&self) -> usize {
        self.dist
    }

    fn op(&self) -> AlignOp {
        self.op
    }
}

type TBMap = HashMap<(usize, usize), Cell>;

fn indices(xs: &[u8], ys: &[u8]) -> impl Iterator<Item = (usize, usize)> {
    (1..=xs.len()).cartesian_product(1..=ys.len())
}

pub fn wagner_fischer(xs: &[u8], ys: &[u8]) -> TBMap {
    let mut edmap = HashMap::new();
    for x in 0..=xs.len() {
        edmap.insert((x, 0), Cell::new(x));
    }

    for y in 0..=ys.len() {
        edmap.insert((0, y), Cell::new(y));
    }

    for (i, j) in indices(xs, ys) {
        if xs[i - 1] == ys[j - 1] {
            let prev = edmap[&(i - 1, j - 1)].dist();
            edmap.insert((i, j), Cell::matches(prev));
        } else {
            let min_op = vec![
                Cell::insertion(edmap[&(i - 1, j)].dist() + 1),
                Cell::deletion(edmap[&(i, j - 1)].dist() + 1),
                Cell::matches(edmap[&(i - 1, j - 1)].dist() + 1),
            ];
            let min_op = min_op
                .iter()
                .cloned()
                .min_by_key(|cell| cell.dist())
                .unwrap();
            edmap.insert((i, j), min_op);
        }
    }
    edmap
}

pub fn distance(xs: &[u8], ys: &[u8], hmap: &TBMap) -> usize {
    hmap[&(xs.len(), ys.len())].dist()
}

pub fn align(xs: &[u8], ys: &[u8], tbmap: &TBMap) -> (String, String) {
    let mut idx = (xs.len(), ys.len());
    let mut xs_acc = Vec::new();
    let mut ys_acc = Vec::new();

    while idx.0 != 0 || idx.1 != 0 {
        match tbmap[&idx].op() {
            AlignOp::Deletion => {
                xs_acc.insert(0, b'-');
                ys_acc.insert(0, ys[idx.1 - 1]);
                idx.1 -= 1;
            }
            AlignOp::Insertion => {
                xs_acc.insert(0, xs[idx.0 - 1]);
                ys_acc.insert(0, b'-');
                idx.0 -= 1;
            }
            AlignOp::Match => {
                xs_acc.insert(0, xs[idx.0 - 1]);
                ys_acc.insert(0, ys[idx.1 - 1]);
                idx.0 -= 1;
                idx.1 -= 1;
            }
            _ => unreachable!(),
        }
    }

    (
        String::from_utf8(xs_acc).unwrap(),
        String::from_utf8(ys_acc).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wagner_fischer() {
        let xs = b"PLEASANTLY";
        let ys = b"MEANLY";
        let tbm = wagner_fischer(xs, ys);
        let actual = distance(xs, ys, &tbm);
        assert_eq!(5, actual);
    }

    #[test]
    fn test_align() {
        let xs = b"PRETTY";
        let ys = b"PRTTEIN";
        let tbm = wagner_fischer(xs, ys);
        let dist = distance(xs, ys, &tbm);
        let (xs_prime, ys_prime) = align(xs, ys, &tbm);
        assert_eq!(dist, 4);
        assert_eq!(String::from("PR-TTEIN"), ys_prime);
        assert_eq!(String::from("PRETTY--"), xs_prime);
    }
}
