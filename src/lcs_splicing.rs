use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

fn empty_hset() -> HashSet<String> {
    let mut hset = HashSet::new();
    hset.insert(String::new());
    hset
}

pub fn longest_common_subsequence(fst: &[u8], snd: &[u8]) -> HashSet<String> {
    let mut indices = Vec::new();

    for i in 1..=fst.len() {
        for j in 1..=snd.len() {
            indices.push((i, j));
        }
    }

    let mut lcs_hmap: HashMap<(usize, usize), HashSet<String>> = HashMap::new();
    for (i, j) in indices {
        if i == 0 || j == 0 {
            lcs_hmap.insert((i, j), empty_hset());
        } else if fst[i - 1] == snd[j - 1] {
            let prev = match lcs_hmap.get(&(i - 1, j - 1)) {
                Some(s) => s.clone(),
                None => empty_hset(),
            };
            let prev = prev
                .iter()
                .cloned()
                .map(|mut pat| {
                    pat.push(fst[i - 1] as char);
                    pat
                }).collect();
            lcs_hmap.insert((i, j), prev);
        } else {
            let left = lcs_hmap
                .get(&(i - 1, j))
                .map_or(empty_hset(), |s| s.clone());
            let right = lcs_hmap
                .get(&(i, j - 1))
                .map_or(empty_hset(), |s| s.clone());
            let longer = left
                .into_iter()
                .chain(right.into_iter())
                .sorted_by_key(|v| v.len());
            let longer = longer.into_iter().group_by(|v| v.len());
            let longer = longer.into_iter().last().unwrap();
            lcs_hmap.insert((i, j), longer.1.collect());
        }
    }
    lcs_hmap[&(fst.len(), snd.len())].iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_subsequence() {
        let actual = longest_common_subsequence(b"AGCAT", b"GAC");
        assert_eq!(
            ["AC".to_owned(), "GC".to_owned(), "GA".to_owned()]
                .iter()
                .cloned()
                .collect::<HashSet<String>>(),
            actual
        );
    }
}
