use itertools::Itertools;

pub fn spliced_motif(target: &[u8], mut query: &[u8]) -> Vec<usize> {
    let mut acc = Vec::new();

    let idx_target = target.iter().zip(1..=target.len());
    for (&ch, idx) in idx_target {
        if ch == query[0] {
            acc.push(idx);
            if query.len() == 1 {
                break;
            } else {
                query = &query[1..];
            }
        }
    }
    acc
}

fn spliced(pattern: &[u8]) -> impl Iterator<Item = &[u8]> {
    (1..=pattern.len()).flat_map(move |len| pattern.windows(len))
}

pub fn shared_spliced_motif(target: &[u8], query: &[u8]) -> String {
    let t_indices = spliced(query).map(|qsub| (true, spliced_motif(target, qsub)));
    let q_indices = spliced(target).map(|tsub| (false, spliced_motif(query, tsub)));
    let (is_target, indices) = q_indices
        .chain(t_indices)
        .max_by_key(|(_, idxs)| idxs.len())
        .unwrap();
    if is_target {
        indices
            .into_iter()
            .map(|idx| target[idx - 1] as char)
            .join("")
    } else {
        indices
            .into_iter()
            .map(|idx| query[idx - 1] as char)
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spliced_motif() {
        let target = b"ACGTACGTGACG";
        let query = b"GTA";
        let indices = spliced_motif(target, query);
        assert_eq!(vec![3, 4, 5], indices);
    }

    #[test]
    fn test_spliced() {
        let target = b"ABC";
        let result: Vec<&[u8]> = spliced(target).collect();
        let answer: Vec<&[u8]> = vec![b"A", b"B", b"C", b"AB", b"BC", b"ABC"];
        assert_eq!(answer, result);
    }

    #[test]
    fn test_shared_spliced_motif() {
        let target = b"AACCTTGG";
        let query = b"ACACTGTGA";
        let shared = shared_spliced_motif(target, query);
        assert_eq!(String::from("ACCTTG"), shared);
    }
}
