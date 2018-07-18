// use itertools::Itertools;

pub fn spliced_motif(target: &[u8], mut query: &[u8]) -> Vec<usize> {
    let mut acc = Vec::new();

    let mut idx_target = target.iter().zip(1 .. target.len());
    while !query.is_empty() {
        let (&ch, idx) = idx_target.next().unwrap();
        if ch == query[0] {
            acc.push(idx);
            query = &query[1..];
        }
    }
    acc
}

pub fn splice(pattern: &[u8]) -> impl Iterator<Item=&[u8]> {
    (0 .. 5).map(move |_| pattern)
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
}
