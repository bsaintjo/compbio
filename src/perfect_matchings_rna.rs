use factorial::Factorial;

pub fn count_perfect_matchings(seq: &[u8]) -> u128 {
    let n_gs = seq.iter().filter(|&b| b == &b'G').count() as u128;
    let n_as = seq.iter().filter(|&b| b == &b'A').count() as u128;
    n_gs.factorial() * n_as.factorial()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count() {
        let seq = b"AGCUAGUCAU";
        assert_eq!(count_perfect_matchings(seq), 12);

        let seq = b"GAUCGGUGCGACACAUAUACUCACGGUCGUAGAACGCCUUAGCCGGGCAUUUCCUUAAACUGCCUAGCAAUGAGGGUU";
        assert_eq!(count_perfect_matchings(seq), 295950609069496384270872084480000000);
    }
}
