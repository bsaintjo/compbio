use log::debug;
use crate::utils;

fn dna_to_protein(dna: &[u8]) -> Option<u8> {
    match dna {
        b"TTT" => Some(b'F'), b"CTT" => Some(b'L'), b"ATT" => Some(b'I'),
        b"GTT" => Some(b'V'), b"TTC" => Some(b'F'), b"CTC" => Some(b'L'),
        b"ATC" => Some(b'I'), b"GTC" => Some(b'V'), b"TTA" => Some(b'L'),
        b"CTA" => Some(b'L'), b"ATA" => Some(b'I'), b"GTA" => Some(b'V'),
        b"TTG" => Some(b'L'), b"CTG" => Some(b'L'), b"ATG" => Some(b'M'),
        b"GTG" => Some(b'V'), b"TCT" => Some(b'S'), b"CCT" => Some(b'P'),
        b"ACT" => Some(b'T'), b"GCT" => Some(b'A'), b"TCC" => Some(b'S'),
        b"CCC" => Some(b'P'), b"ACC" => Some(b'T'), b"GCC" => Some(b'A'),
        b"TCA" => Some(b'S'), b"CCA" => Some(b'P'), b"ACA" => Some(b'T'),
        b"GCA" => Some(b'A'), b"TCG" => Some(b'S'), b"CCG" => Some(b'P'),
        b"ACG" => Some(b'T'), b"GCG" => Some(b'A'), b"TAT" => Some(b'Y'),
        b"CAT" => Some(b'H'), b"AAT" => Some(b'N'), b"GAT" => Some(b'D'),
        b"TAC" => Some(b'Y'), b"CAC" => Some(b'H'), b"AAC" => Some(b'N'),
        b"GAC" => Some(b'D'), b"TAA" => None, b"CAA" => Some(b'Q'),
        b"AAA" => Some(b'K'), b"GAA" => Some(b'E'), b"TAG" => None,
        b"CAG" => Some(b'Q'), b"AAG" => Some(b'K'), b"GAG" => Some(b'E'),
        b"TGT" => Some(b'C'), b"CGT" => Some(b'R'), b"AGT" => Some(b'S'),
        b"GGT" => Some(b'G'), b"TGC" => Some(b'C'), b"CGC" => Some(b'R'),
        b"AGC" => Some(b'S'), b"GGC" => Some(b'G'), b"TGA" => None,
        b"CGA" => Some(b'R'), b"AGA" => Some(b'R'), b"GGA" => Some(b'G'),
        b"TGG" => Some(b'W'), b"CGG" => Some(b'R'), b"AGG" => Some(b'R'),
        b"GGG" => Some(b'G'), _ => unreachable!(),
    }
}


fn find_starts(dna: &[u8]) -> Vec<usize> {
    let mut acc = Vec::new();
    for (idx, codon) in dna.windows(3).enumerate() {
        if let Some(b'M') = dna_to_protein(codon) {
            acc.push(idx);
        }
    }
    acc
}


fn expand_start(idx: usize, dna: &[u8]) -> Option<Vec<u8>> {
    let mut acc = Vec::new();
    let dna = &dna[idx..];
    for codon in ExactChunks::new(dna, 3) {
        if let Some(aa) = dna_to_protein(codon) {
            acc.push(aa);
        } else {
            return Some(acc);
        }
    }
    None
}

fn expand_orf(dna: &[u8]) -> Vec<Vec<u8>> {
    let starts = find_starts(dna);
    let mut acc = Vec::new();
    for start in starts {
        if let Some(orf) = expand_start(start, dna) {
            acc.push(orf)
        }
    }
    acc
}


pub fn find_orfs2(dna: &[u8]) -> Vec<Vec<u8>>{
    let mut acc = Vec::new();
    acc.append(&mut expand_orf(dna));
    let rev_dna = rev_comp(dna);
    acc.append(&mut expand_orf(&rev_dna));
    acc.sort();
    acc.dedup();
    acc
}


fn translate(dna: &[u8]) -> Vec<Option<u8>> {
    let codons = ExactChunks::new(dna, 3);
    let mut acc = Vec::new();
    for codon in codons {
        acc.push(dna_to_protein(codon))
    }
    acc
}

fn split_orfs(dna: &[u8]) -> Vec<Vec<u8>> {
    let translated = translate(dna);
    debug!("DNA: {:?}", utils::u8_to_string(dna));
    debug!("TRANSLATED: {:?}", translated.iter().map(|x| x.map(|y| y as char)).collect::<Vec<Option<char>>>());

    let mut acc = Vec::new();
    for protein in translated.split(|codon| codon.is_none()) {
        let unwrapped: Vec<u8> = protein
            .iter()
            .skip_while(|&&x| x != Some(b'M')) // Trim until start codon
            .map(|x| x.unwrap())
            .collect();
        if unwrapped.len() == 0 {
            continue
        } else {
            acc.push(unwrapped);
        }
    }
    acc
}

fn all_frames(dna: &[u8]) -> Vec<Vec<u8>> {
    let mut fst = split_orfs(dna);
    let mut snd = split_orfs(&dna[1..]);
    let mut thrd = split_orfs(&dna[2..]);
    fst.append(&mut snd);
    fst.append(&mut thrd);
    fst
}

fn rev_comp(dna: &[u8]) -> Vec<u8> {
    dna.iter()
        .rev()
        .map(|x| match x {
            b'A' => b'T',
            b'C' => b'G',
            b'G' => b'C',
            b'T' => b'A',
            _ => unreachable!(),
        })
        .collect()
}

pub fn find_orfs(dna: &[u8]) -> Vec<Vec<u8>> {
    let mut fwd = all_frames(dna);
    let mut rev = all_frames(&rev_comp(dna));
    fwd.append(&mut rev);
    fwd
}

// Can't import the experimental api now for some reason
// I'll reimplement a simpler version here until it works
pub struct ExactChunks<'a, T: 'a> {
    slice: &'a [T],
    chunk_size: usize,
}

impl<'a, T> ExactChunks<'a, T> {
    pub fn new(slice: &'a [T], chunk_size: usize) -> Self {
        ExactChunks { slice, chunk_size }
    }
}

impl<'a, T> Iterator for ExactChunks<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.len() < self.chunk_size {
            None
        } else {
            let chunk = &self.slice[..self.chunk_size];
            self.slice = &self.slice[self.chunk_size..];
            Some(chunk)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_chunks() {
        let dna = b"ABCDEFG";
        let mut chunks = ExactChunks::new(dna, 3);
        assert_eq!(Some(b"ABC" as &[u8]), chunks.next());
        assert_eq!(Some(b"DEF" as &[u8]), chunks.next());
        assert_eq!(None, chunks.next());
    }

    #[test]
    fn test_rev_comp() {
        let dna = b"GTCC";
        let rc = rev_comp(dna);
        assert_eq!(b"GGAC" as &[u8], rc.as_slice());
    }

    #[test]
    fn test_split_orf() {
        let dna = b"CCCATGTGAATGCCC";
        let orfs = split_orfs(dna);
        assert_eq!(&orfs[0], &[b'M']);
        assert_eq!(&orfs[1], &[b'M', b'P']);
    }
}
