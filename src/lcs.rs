use itertools::{iproduct, Itertools};
use ndarray::{Array2, ArrayD, IntoDimension};
use std::collections::HashMap;

pub fn lc_suffix<'a>(xs: &'a [u8], ys: &'a [u8]) -> &'a [u8] {
    let mut lcs_idx = 0;
    let mut lcs_length = 0;
    let mut matrix = Array2::<usize>::zeros((xs.len() + 1, ys.len() + 1));
    let indices = iproduct![1..=xs.len(), 1..=ys.len()];
    for (x_idx, y_idx) in indices {
        unsafe {
            if xs.get_unchecked(x_idx - 1) == ys.get_unchecked(y_idx - 1) {
                matrix[[x_idx, y_idx]] = 1 + matrix[[x_idx - 1, y_idx - 1]];
            }
        }
        if matrix[[x_idx, y_idx]] > lcs_length {
            lcs_length = matrix[[x_idx, y_idx]];
            lcs_idx = x_idx;
        }
    }
    &xs[lcs_idx - lcs_length..lcs_idx]
}

pub fn kcommon<'a>(xss: &'a [&'a [u8]]) -> &'a [u8] {
    let mut lcs_idx = 0;
    let mut lcs_length = 0;
    let mut matrix =
        ArrayD::<usize>::zeros(xss.iter().map(|xs| xs.len() + 1).collect::<Vec<usize>>());
    let indices: Vec<Vec<usize>> = xss
        .iter()
        .map(|xs| 1..=xs.len())
        .multi_cartesian_product()
        .collect();

    for index in indices {
        let mut iter = xss
            .iter()
            .zip(index.iter())
            .map(|(xs, &idx)| unsafe { xs.get_unchecked(idx - 1) });
        let init = iter.next().unwrap();
        let dim = index.clone().into_dimension();

        if iter.all(|x| x == init) {
            let prev = index
                .iter()
                .map(|x| x - 1)
                .collect::<Vec<usize>>()
                .into_dimension();
            matrix[&dim] = 1 + matrix[prev];
        }

        if matrix[&dim] > lcs_length {
            lcs_length = matrix[dim];
            lcs_idx = index[0];
        }
    }

    &xss[0][lcs_idx - lcs_length..lcs_idx]
}

pub fn kcommon_ht<'a>(xss: &'a [&'a [u8]]) -> &'a [u8] {
    let mut lcs_idx = 0;
    let mut lcs_length = 0;
    let mut hmap = HashMap::new();
    let indices: Vec<Vec<usize>> = xss
        .iter()
        .map(|xs| 1..=xs.len())
        .multi_cartesian_product()
        .collect();

    for index in indices {
        let mut iter = xss
            .iter()
            .zip(index.iter())
            .map(|(xs, &idx)| unsafe { xs.get_unchecked(idx - 1) });
        let init = iter.next().unwrap();

        if iter.all(|x| x == init) {
            let prev = index.iter().map(|x| x - 1).collect::<Vec<usize>>();
            let new_length = 1 + hmap.get(&prev).unwrap_or(&0);
            hmap.insert(index.clone(), new_length);

            if new_length > lcs_length {
                lcs_length = new_length;
                lcs_idx = index[0];
            }
        }
    }

    &xss[0][lcs_idx - lcs_length..lcs_idx]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs() {
        let xs = b"ACGTACGT";
        let ys = b"ACCGTATA";
        let answer = lc_suffix(xs, ys);
        assert_eq!(b"CGTA", answer);
    }

    #[test]
    fn test_kcommon() {
        let xs = b"ABABC";
        let ys = b"BABCA";
        let zs = b"ABCBA";
        let strs: &[&[u8]] = &[xs, ys, zs];
        let answer = kcommon(strs);
        assert_eq!(b"ABC", answer);
    }

    #[test]
    fn test_kcommon2() {
        let xs = b"GATTACA";
        let ys = b"TAGACCA";
        let zs = b"ATACA";
        let strs: &[&[u8]] = &[xs, ys, zs];
        let answer = kcommon(strs);
        assert_eq!(b"TA", answer);
    }

    #[test]
    fn test_kcommon_ht() {
        let xs = b"ABABC";
        let ys = b"BABCA";
        let zs = b"ABCBA";
        let strs: &[&[u8]] = &[xs, ys, zs];
        let answer = kcommon_ht(strs);
        assert_eq!(b"ABC", answer);
    }
}
