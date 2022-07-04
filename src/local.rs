use std::collections::HashMap;
use std::collections::HashSet;

use bio::scores::pam250;
use itertools::iproduct;
use log::info;

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
        *self.dirs.iter().next().unwrap()
    }
}

type AlignmentMatrix = HashMap<(usize, usize), Cell>;

pub struct Builder<'a> {
    fst: &'a [u8],
    snd: &'a [u8],
    linear_gap_penalty: isize,
}

impl<'a> Builder<'a> {
    pub fn new(fst: &'a [u8], snd: &'a [u8], linear_gap_penalty: isize) -> Self {
        Builder {
            fst,
            snd,
            linear_gap_penalty,
        }
    }

    fn initialize_alignment_matrix(&self) -> AlignmentMatrix {
        let mut mat = HashMap::new();
        mat.insert((0, 0), Cell::new(0));

        for idx in 1..=self.fst.len() {
            mat.insert(
                (idx, 0usize),
                Cell::new_with_dir(idx as isize * self.linear_gap_penalty, Direction::left()),
            );
        }

        for idx in 1..=self.snd.len() {
            mat.insert(
                (0usize, idx),
                Cell::new_with_dir(idx as isize * self.linear_gap_penalty, Direction::top()),
            );
        }
        mat
    }

    pub fn align(&self) -> LocalAlignment {
        let mut mat = self.initialize_alignment_matrix();
        let indices = iproduct![1..=self.fst.len(), 1..=self.snd.len()];
        for (fst_idx, snd_idx) in indices {
            info!(
                "Determining aligment for cell at pos {}, {}",
                fst_idx, snd_idx
            );
            info!("Indexing top cell at pos {}, {}", fst_idx - 1, snd_idx);
            let top = Builder::score_idx(fst_idx - 1, snd_idx, &mat) + self.linear_gap_penalty;
            info!("Indexing left cell at pos {}, {}", fst_idx, snd_idx - 1);
            let left = Builder::score_idx(fst_idx, snd_idx - 1, &mat) + self.linear_gap_penalty;
            info!(
                "Indexing top left cell at pos {}, {}",
                fst_idx - 1,
                snd_idx - 1
            );
            let matching = Builder::score_idx(fst_idx - 1, snd_idx - 1, &mat)
                + self.scoring_matrix(fst_idx, snd_idx);
            let cell_result = Cell::from_cell_max(&[
                (top, Direction::Top),
                (left, Direction::Left),
                (matching, Direction::TopLeft),
            ]);
            info!(
                "Inserting cell with score {} as pos {}, {}",
                cell_result.score(),
                fst_idx,
                snd_idx
            );
            Builder::insert((fst_idx, snd_idx), cell_result, &mut mat);
        }
        let (fst_aligned, snd_aligned) = self.traceback(&mat);
        let max_score = self.maximum_alignment_score(&mat);
        LocalAlignment::new(fst_aligned, snd_aligned, mat, max_score)
    }

    fn insert(idxs: (usize, usize), cell: Cell, mat: &mut AlignmentMatrix) {
        mat.insert(idxs, cell);
    }

    fn scoring_matrix(&self, fst_idx: usize, snd_idx: usize) -> isize {
        let fst_char = self.fst[fst_idx - 1];
        let snd_char = self.snd[snd_idx - 1];
        pam250::pam250(fst_char, snd_char) as isize
    }

    fn score_idx(fst_idx: usize, snd_idx: usize, mat: &AlignmentMatrix) -> isize {
        Builder::cell_idx(fst_idx, snd_idx, mat).score()
    }

    fn cell_idx(fst_idx: usize, snd_idx: usize, mat: &AlignmentMatrix) -> &Cell {
        &mat[&(fst_idx, snd_idx)]
    }

    fn maximum_alignment_score(&self, _mat: &AlignmentMatrix) -> isize {
        unimplemented!()
    }

    fn traceback(&self, mat: &AlignmentMatrix) -> (Vec<u8>, Vec<u8>) {
        let mut pos = (self.fst.len(), self.snd.len());
        // TODO Use VecDeque instead to add letters in the correct order
        let mut fst_align = Vec::new();
        let mut snd_align = Vec::new();

        while pos.0 != 0 || pos.1 != 0 {
            match Builder::cell_idx(pos.0, pos.1, mat).get_any_dir() {
                Direction::Top => {
                    fst_align.push(b'-');
                    snd_align.push(self.snd[pos.1 - 1]);
                    pos.1 -= 1;
                }
                Direction::Left => {
                    fst_align.push(self.fst[pos.0 - 1]);
                    snd_align.push(b'-');
                    pos.0 -= 1;
                }
                Direction::TopLeft => {
                    fst_align.push(self.fst[pos.0 - 1]);
                    snd_align.push(self.snd[pos.1 - 1]);
                    pos.0 -= 1;
                    pos.1 -= 1;
                }
            }
        }
        (fst_align, snd_align)
    }
}

pub struct LocalAlignment {
    pub fst_aligned: Vec<u8>,
    pub snd_aligned: Vec<u8>,
    pub max_score: isize,
    mat: AlignmentMatrix,
}

impl LocalAlignment {
    fn new(
        fst_aligned: Vec<u8>,
        snd_aligned: Vec<u8>,
        mat: AlignmentMatrix,
        max_score: isize,
    ) -> Self {
        LocalAlignment {
            fst_aligned,
            snd_aligned,
            mat,
            max_score,
        }
    }
}
