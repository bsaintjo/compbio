use crate::suffix_array;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::Chars;

pub struct BWT {
    pub bwt: String,
}

impl BWT {
    pub fn new(text: String) -> BWT {
        let sarray = suffix_array::SuffixArray::new(text);
        BWT {
            bwt: sarray
                .index_array()
                .iter()
                .map(|&idx| {
                    if idx == 0 {
                        "$"
                    } else {
                        &sarray.text()[idx - 1..idx]
                    }
                })
                .collect(),
        }
    }

    pub fn fst_column(&self) -> String {
        let mut sorted = self.bwt.clone().into_bytes();
        sorted.sort();
        String::from_utf8(sorted).unwrap()
    }

    pub fn invert(&self) -> String {
        let bwt_hmap = column_hash_map(&self.bwt);
        let mcolumn: Vec<_> = uniq_count(&self.fst_column()).collect();

        let mut inverted = String::with_capacity(self.bwt.len());
        let mut idx_char = mcolumn[0];
        inverted.push(idx_char.1);

        for _ in 0..self.bwt.len() {
            let next_idx = bwt_hmap[&idx_char.1][idx_char.0];
            idx_char = mcolumn[next_idx];
            inverted.push(idx_char.1);
        }

        String::from(&inverted[1..])
    }
}

pub fn column_hash_map(text: &str) -> HashMap<char, Vec<usize>> {
    let mut hmap: HashMap<char, Vec<usize>> = HashMap::new();
    for (idx, ch) in text.chars().enumerate() {
        match hmap.entry(ch) {
            Entry::Occupied(oe) => {
                oe.into_mut().push(idx);
            }
            Entry::Vacant(ve) => {
                ve.insert(vec![idx]);
            }
        }
    }
    hmap
}

pub struct UniqCount<'a> {
    text_iter: Chars<'a>,
    seen: HashMap<char, usize>,
}

impl<'a> Iterator for UniqCount<'a> {
    type Item = (usize, char);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.text_iter.next() {
            match self.seen.entry(ch) {
                Entry::Occupied(mut oe) => {
                    *oe.get_mut() += 1;
                    Some((*oe.get(), ch))
                }
                Entry::Vacant(ve) => {
                    ve.insert(0);
                    Some((0, ch))
                }
            }
        } else {
            None
        }
    }
}

pub fn uniq_count(text: &str) -> UniqCount {
    let text_iter = text.chars();
    let hmap = HashMap::new();
    UniqCount {
        text_iter,
        seen: hmap,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bwt() {
        let text = String::from("panamabananas$");
        let tfed = BWT::new(text);
        assert_eq!(tfed.bwt, String::from("smnpbnnaaaaa$a"));
    }

    #[test]
    fn test_fst_column() {
        let text = String::from("panamabananas$");
        let tfed = BWT::new(text);
        assert_eq!(tfed.fst_column(), String::from("$aaaaaabmnnnps"));
    }

    #[test]
    fn test_column_mapping() {
        let xs = String::from("smnpbnnaaaaa$a");
        let mut ans: HashMap<char, Vec<usize>> = HashMap::new();
        for (idx, ch) in xs.chars().enumerate() {
            match ans.entry(ch) {
                Entry::Occupied(oe) => {
                    oe.into_mut().push(idx);
                }
                Entry::Vacant(ve) => {
                    ve.insert(vec![idx]);
                }
            }
        }

        let mut corr: HashMap<char, Vec<usize>> = HashMap::new();
        corr.insert('$', vec![12]);
        corr.insert('a', vec![7, 8, 9, 10, 11, 13]);
        corr.insert('b', vec![4]);
        corr.insert('m', vec![1]);
        corr.insert('n', vec![2, 5, 6]);
        corr.insert('p', vec![3]);
        corr.insert('s', vec![0]);

        assert_eq!(ans.len(), corr.len());
        assert_eq!(ans, corr);
    }

    #[test]
    fn test_uniq_count() {
        let text = String::from("$aaaaaabmnnnps");
        let uniq_counted: Vec<_> = uniq_count(&text).collect();
        assert_eq!(
            uniq_counted,
            vec![
                (0, '$'),
                (0, 'a'),
                (1, 'a'),
                (2, 'a'),
                (3, 'a'),
                (4, 'a'),
                (5, 'a'),
                (0, 'b'),
                (0, 'm'),
                (0, 'n'),
                (1, 'n'),
                (2, 'n'),
                (0, 'p'),
                (0, 's'),
            ]
        );
    }

    #[test]
    fn test_inversion() {
        let text = String::from("panamabananas$");
        let tfed = BWT::new(text.clone());
        assert_eq!(text, tfed.invert());
    }
}
