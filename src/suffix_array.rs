use crate::suffix_tree;

pub struct SuffixArray {
    text: String,
    iarray: Vec<usize>,
}

impl SuffixArray {
    pub fn new(text: String) -> Self {
        let iarray = {
            let mut xs = suffix_tree::suffixes(&text)
                .enumerate()
                .collect::<Vec<(usize, &str)>>();
            xs.sort_by_key(|&(_, s)| s);
            xs.iter().map(|&(i, _)| i).collect()
        };
        SuffixArray { text, iarray }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn index_array(&self) -> &[usize] {
        &self.iarray
    }

    pub fn get_suffix(&self, idx: usize) -> &str {
        &self.text[idx..]
    }

    pub fn pattern_match(&self, pattern: &str) -> Option<(usize, usize)> {
        let mut min_index = 0;
        let mut max_index = self.text.len();

        while min_index < max_index {
            let mid_index = (min_index + max_index) / 2;
            if pattern > self.get_suffix(self.iarray[mid_index]) {
                min_index = mid_index + 1;
            } else {
                max_index = mid_index;
            }
        }

        let first = min_index;
        max_index = self.text.len();

        while min_index < max_index {
            let mid_index = (min_index + max_index) / 2;
            if !self.get_suffix(self.iarray[mid_index]).starts_with(pattern) {
                max_index = mid_index;
            } else {
                min_index = mid_index + 1;
            }
        }

        let last = max_index - 1;
        if first > last {
            None
        } else {
            Some((first, last))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_array_new() {
        let text = String::from("panamabananas$");
        let sarray = SuffixArray::new(text);
        assert_eq!(
            vec![13, 5, 3, 1, 7, 9, 11, 6, 4, 2, 8, 10, 0, 12],
            sarray.index_array()
        )
    }

    #[test]
    fn test_suffix_array_new2() {
        let text = String::from("AACGATAGCGGTAGA$");
        let sarray = SuffixArray::new(text);
        assert_eq!(
            vec![15, 14, 0, 1, 12, 6, 4, 2, 8, 13, 3, 7, 9, 10, 11, 5],
            sarray.index_array()
        )
    }

    #[test]
    fn test_pattern_match() {
        let text = String::from("panamabananas$");
        let pattern = "ana";
        let sarray = SuffixArray::new(text);
        assert_eq!(sarray.pattern_match(pattern), Some((3, 5)));
    }

    #[test]
    fn test_pattern_match2() {
        let text = String::from("AATCGGGTTCAATCGGGGT$");
        let pattern = "ATCG";
        let sarray = SuffixArray::new(text);
        assert_eq!(sarray.pattern_match(pattern), Some((3, 4)));
    }
}
