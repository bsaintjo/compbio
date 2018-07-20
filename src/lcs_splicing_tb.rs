use std::collections::HashMap;
use std::collections::HashSet;

use log::{debug, log};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum TB {
    Up,
    Left,
    Diagonal,
}

#[derive(Clone, Debug)]
pub struct Cell {
    len: usize,
    dirs: HashSet<TB>,
}

impl Cell {
    fn new() -> Self {
        Cell {
            len: 0,
            dirs: HashSet::new(),
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_diagonal(&self) -> bool {
        let mut diag_hashset = HashSet::new();
        diag_hashset.insert(TB::Diagonal);
        self.dirs == diag_hashset
    }

    fn next_dir(&self) -> TB {
        self.dirs.iter().next().unwrap().clone()
    }

    fn add_one(&mut self) {
        self.len += 1;
    }

    fn add_dir(&mut self, dir: TB) {
        self.dirs.insert(dir);
    }

    fn clone_wo_dirs(&self) -> Self {
        Cell {
            len: self.len,
            dirs: HashSet::new(),
        }
    }
}

type TracebackMap = HashMap<(usize, usize), Cell>;

pub fn trace(fst: &[u8], snd: &[u8], tbm: &TracebackMap) -> Vec<String> {
    let mut init = (fst.len() - 1, snd.len() - 1);
    let mut acc = vec![String::new()];
    while init.0 != 0 && init.1 != 0 {
        debug!("init.0 {:?}", init.0);
        debug!("init.1 {:?}", init.1);
        debug!("tbm {:?}", tbm);
        let cell = &tbm[&init];
        if cell.is_diagonal() {
            for s in &mut acc {
                let mut tmp = (fst[init.0 - 1] as char).to_string();
                tmp.push_str(&s);
                *s = tmp;
            }
            init.0 -= 1;
            init.1 -= 1;
        } else {
            match cell.next_dir() {
                TB::Left => init.0 -= 1,
                TB::Up => init.1 -= 1,
                TB::Diagonal => unreachable!(),
            }
        }
    }
    acc
}

pub fn longest_common_subsequence(fst: &[u8], snd: &[u8]) -> TracebackMap {
    let mut indices = Vec::new();

    for i in 1..=fst.len() {
        for j in 1..=snd.len() {
            indices.push((i, j));
        }
    }

    let mut lcs_hmap: TracebackMap = HashMap::new();
    for (i, j) in indices {
        if i == 0 || j == 0 {
            lcs_hmap.insert((i, j), Cell::new());
        } else if fst[i - 1] == snd[j - 1] {
            let mut prev = match lcs_hmap.get(&(i - 1, j - 1)) {
                Some(s) => s.clone_wo_dirs(),
                None => Cell::new(),
            };
            prev.add_dir(TB::Diagonal);
            prev.add_one();
            lcs_hmap.insert((i, j), prev);
        } else {
            let mut left = lcs_hmap
                .get(&(i - 1, j))
                .map_or(Cell::new(), |s| s.clone_wo_dirs());
            let mut up = lcs_hmap
                .get(&(i, j - 1))
                .map_or(Cell::new(), |s| s.clone_wo_dirs());
            let longer = if left.len() > up.len() {
                left.add_dir(TB::Left);
                left
            } else if up.len() > left.len() {
                up.add_dir(TB::Up);
                up
            } else {
                left.add_dir(TB::Left);
                left.add_dir(TB::Up);
                left
            };
            lcs_hmap.insert((i, j), longer);
        }
    }
    lcs_hmap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_subsequence() {
        let fst = b"AGCAT";
        let snd = b"GAC";
        let tbm = longest_common_subsequence(fst, snd);
        let actual = trace(fst, snd, &tbm);
        assert_eq!(
            vec!["AC".to_owned(), "GC".to_owned(), "GA".to_owned()],
            actual
        );
    }
}
