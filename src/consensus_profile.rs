use itertools::Itertools;
use std::fmt::Display;

use itertools::izip;

pub struct Profile {
    a: Vec<usize>,
    c: Vec<usize>,
    g: Vec<usize>,
    t: Vec<usize>,
}

impl Profile {
    pub fn new() -> Self {
        let a = vec![0; 1000];
        let c = vec![0; 1000];
        let g = vec![0; 1000];
        let t = vec![0; 1000];
        Profile { a, c, g, t }
    }

    pub fn add_seq(&mut self, seq: &[u8]) {
        for (idx, &base) in seq.iter().enumerate() {
            match base {
                b'A' => self.a[idx] += 1,
                b'C' => self.c[idx] += 1,
                b'G' => self.g[idx] += 1,
                b'T' => self.t[idx] += 1,
                _ => todo!(),
            }
        }
    }

    pub fn consensus(&self) -> String {
        let bases = ['A', 'C', 'G', 'T'];
        let mut acc = String::new();
        for base_counts in izip!(&self.a, &self.c, &self.g, &self.t) {
            match base_counts {
                (0, 0, 0, 0) => break,
                (a, c, g, t) => {
                    let x = [a, c, g, t]
                        .into_iter()
                        .enumerate()
                        .max_by_key(|(_, &b)| b)
                        .map(|(i, _)| i)
                        .unwrap();
                    acc.push(bases[x]);
                }
            }
        }
        acc
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let consensus = self.consensus();
        writeln!(f, "{}", consensus)?;
        let a = self.a.iter().take(consensus.len()).join(" ");
        writeln!(f, "A: {}", a)?;
        let c = self.c.iter().take(consensus.len()).join(" ");
        writeln!(f, "C: {}", c)?;
        let g = self.g.iter().take(consensus.len()).join(" ");
        writeln!(f, "G: {}", g)?;
        let t = self.t.iter().take(consensus.len()).join(" ");
        write!(f, "T: {}", t)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_consensus() {
        let xs = vec![
            b"ATCCAGCT",
            b"GGGCAACT",
            b"ATGGATCT",
            b"AAGCAACC",
            b"TTGGAACT",
            b"ATGCCATT",
            b"ATGGCACT",
        ];

        let mut profile = Profile::new();
        xs.into_iter().for_each(|x| profile.add_seq(x));
        let consensus = profile.consensus();
        assert_eq!(consensus, String::from("ATGCAACT"));
    }
}
