use bio::scores::blosum62;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Top,
    TopLeft,
    Left,
}

impl Direction {
    fn top() -> HashSet<Direction> {
        let mut acc = HashSet::new();
        acc.insert(Direction::Top);
        acc
    }

    fn left() -> HashSet<Direction> {
        let mut acc = HashSet::new();
        acc.insert(Direction::Left);
        acc
    }
}

struct Cell {
    score: isize,
    dirs: HashSet<Direction>,
}

impl Cell {
    fn new(score: isize) -> Self {
        let dirs = HashSet::new();
        Cell { score, dirs }
    }

    fn new_with_dir(score: isize, dirs: HashSet<Direction>) -> Self {
        Cell { score, dirs }
    }

    fn score(&self) -> isize {
        self.score
    }

    fn from_cell_max(iter_cell: &[(isize, Direction)]) -> Self {
        let mut iter_cell = iter_cell.into_iter();
        let first = iter_cell.next().unwrap();
        let mut max_score = first.0;
        let mut acc = HashSet::new();
        acc.insert(first.1);

        for &(score, dir) in iter_cell {
            if score == max_score {
                acc.insert(dir);
            } else if score > max_score {
                max_score = score;
                acc.clear();
                acc.insert(dir);
            }
        }

        Cell {
            score: max_score,
            dirs: acc,
        }
    }

    fn get_any_dir(&self) -> Direction {
        self.dirs.iter().next().unwrap().clone()
    }
}

type AlignmentMatrix = HashMap<(usize, usize), Cell>;

pub struct GlobalAlignment {
    fst: Vec<u8>,
    snd: Vec<u8>,
    linear_gap_penalty: isize,
    mat: AlignmentMatrix,
}

impl GlobalAlignment {
    pub fn new(fst: Vec<u8>, snd: Vec<u8>) -> Self {
        let mut mat = HashMap::new();
        let linear_gap_penalty = -5isize;
        mat.insert((0, 0), Cell::new(0));

        for idx in 1..=fst.len() {
            mat.insert(
                (idx, 0usize),
                Cell::new_with_dir(idx as isize * linear_gap_penalty, Direction::left()),
            );
        }

        for idx in 1..=snd.len() {
            mat.insert(
                (0usize, idx),
                Cell::new_with_dir(idx as isize * linear_gap_penalty, Direction::top()),
            );
        }

        GlobalAlignment {
            fst,
            snd,
            linear_gap_penalty,
            mat,
        }
    }

    fn insert(&mut self, idxs: (usize, usize), cell: Cell) {
        self.mat.insert(idxs, cell).unwrap();
    }

    fn mat_idx(&self, fst_idx: usize, snd_idx: usize) -> isize {
        self.cell_idx(fst_idx, snd_idx).score()
    }

    fn cell_idx(&self, fst_idx: usize, snd_idx: usize) -> &Cell {
        &self.mat[&(fst_idx, snd_idx)]
    }

    fn scoring_matrix(&self, fst_idx: usize, snd_idx: usize) -> isize {
        let fst_char = self.fst[fst_idx - 1];
        let snd_char = self.snd[snd_idx - 1];
        blosum62::blosum62(fst_char, snd_char) as isize
    }

    pub fn align(&mut self) {
        for (fst_idx, snd_idx) in (2..=self.fst.len()).zip(2..=self.snd.len()) {
            let top = self.mat_idx(fst_idx - 1, snd_idx) + self.linear_gap_penalty;
            let left = self.mat_idx(fst_idx, snd_idx - 1) + self.linear_gap_penalty;
            let matching =
                self.mat_idx(fst_idx - 1, snd_idx - 1) + self.scoring_matrix(fst_idx, snd_idx);
            let cell_result = Cell::from_cell_max(&[
                (top, Direction::Top),
                (left, Direction::Left),
                (matching, Direction::TopLeft),
            ]);
            self.insert((fst_idx, snd_idx), cell_result);
        }
    }

    pub fn maximum_alignment_score(&self) -> isize {
        self.mat_idx(self.fst.len(), self.snd.len())
    }

    pub fn traceback(&self) -> (Vec<u8>, Vec<u8>) {
        let mut pos = (self.fst.len(), self.snd.len());
        let mut fst_align = Vec::new();
        let mut snd_align = Vec::new();

        while pos.0 != 0 && pos.1 != 0 {
            match self.cell_idx(pos.0, pos.1).get_any_dir() {
                Direction::Top => {
                    fst_align.push(b'-');
                    snd_align.push(self.snd[pos.1]);
                    pos.1 -= 1;
                }
                Direction::Left => {
                    fst_align.push(self.fst[pos.0]);
                    snd_align.push(b'-');
                    pos.0 -= 1;
                }
                Direction::TopLeft => {
                    fst_align.push(self.fst[pos.0]);
                    snd_align.push(self.snd[pos.1]);
                    pos.0 -= 1;
                    pos.1 -= 1;
                }
            }
        }
        (fst_align, snd_align)
    }
}
