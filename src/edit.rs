use itertools::Itertools;
use std::collections::HashMap;

fn indices(xs: &[u8], ys: &[u8]) -> impl Iterator<Item = (usize, usize)> {
    (1..=xs.len()).cartesian_product(1..=ys.len())
}

pub fn wagner_fischer(xs: &[u8], ys: &[u8]) -> usize {
    let mut edmap = HashMap::new();
    for x in 0..=xs.len() {
        edmap.insert((x, 0), x);
    }

    for y in 0..=ys.len() {
        edmap.insert((0, y), y);
    }

    for (i, j) in indices(xs, ys) {
        if xs[i-1] == ys[j-1] {
            let prev = edmap[&(i - 1, j - 1)];
            edmap.insert((i, j), prev);
        } else {
            let min_op = vec![
                edmap[&(i - 1, j)],
                edmap[&(i, j - 1)],
                edmap[&(i - 1, j - 1)],
            ];
            let min_op = min_op.iter().min().unwrap();
            edmap.insert((i, j), min_op + 1);
        }
    }
    edmap[&(xs.len(), ys.len())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wagner_fischer() {
        let xs = b"PLEASANTLY";
        let ys = b"MEANLY";
        let actual = wagner_fischer(xs, ys);
        assert_eq!(5, actual);
    }
}
