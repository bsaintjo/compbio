use itertools::Itertools;
use std::collections::HashMap;

pub fn longest_common_subsequence(fst: &[u8], snd: &[u8]) -> Vec<String> {
    let mut indices = Vec::new();

    for i in 1..fst.len() + 1 {
        for j in 1..snd.len() + 1 {
            indices.push((i, j));
        }
    }

    let mut lcs_hmap: HashMap<(usize, usize), Vec<String>> = HashMap::new();
    for (i, j) in indices {
        if i == 0 || j == 0 {
            lcs_hmap.insert((i, j), vec![String::new()]);
        } else if fst[i] == snd[j] {
            let mut prev = match lcs_hmap.get(&(i - 1, j - 1)) {
                Some(s) => s.clone(),
                None => vec![String::new()],
            };
            for pat in &mut prev {
                pat.push(fst[i] as char)
            }
            lcs_hmap.insert((i, j), prev);
        } else {
            let mut left = lcs_hmap
                .get(&(i - 1, j))
                .map_or(vec![String::new()], |s| s.clone());
            let mut right = lcs_hmap
                .get(&(i, j - 1))
                .map_or(vec![String::new()], |s| s.clone());
            let longer = left
                .into_iter()
                .chain(right.into_iter())
                .sorted_by_key(|v| v.len());
            let longer = longer.into_iter().group_by(|v| v.len());
            let longer = longer.into_iter().last().unwrap();
            lcs_hmap.insert((i, j), longer.1.collect_vec());
        }
    }
    lcs_hmap.get(&(fst.len(), snd.len())).unwrap().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_subsequence() {

    }
}
