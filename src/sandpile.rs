use std::collections::VecDeque;
use crate::canvas::*;

#[derive(Clone, Copy)]
struct Grain {
    hgt: usize,
    scheduled: bool,
}

impl Grain {
    pub fn new() -> Self {
        Grain {
            hgt: 0,
            scheduled: false,
        }
    }
}

impl Colorize for Grain {
    fn color(&self) -> Color {
        match self.hgt {
            0 => (0, 0, 0),
            1 => (13, 4, 0),
            2 => (25, 9, 0),
            3 => (25, 20, 0),
            _ => (0, 0, 0),
        }
    }
}

pub struct Sandpile {
    field: Canvas<Grain>,
    hgt: usize,
    wth: usize,
    schedule: VecDeque<(usize, usize)>,
    cnt: usize,
}


impl Sandpile {
    pub fn new(i: usize, j: usize) -> Self {
        Sandpile {
            field: Canvas::new(i, j, Grain::new()),
            hgt: i,
            wth: j,
            schedule: VecDeque::new(),
            cnt: 0,
        }
    }

    fn is_valid_move(&self, i: usize, j: usize, mvi: isize, mvj: isize) -> bool {
        match mvi {
            -1 => if i == 0 { return false; },
            1 => if i == self.hgt - 1 { return false; },
            _ => ()
        }
        match mvj {
            -1 => if j == 0 { return false; },
            1 => if j == self.hgt - 1 { return false; },
            _ => ()
        }
        true
    }

    fn index_move(&self, i: usize, j: usize, mvi: isize, mvj: isize) -> [usize; 2] {
        let (mut i, mut j) = (i, j);
        match mvi {
            -1 => if i == 0 { i = self.hgt - 1; } else { i -= 1;},
            1 => if i == self.hgt - 1 { i = 0 } else { i += 1; },
            0 => (),
            _ => panic!("({}, {}) is not a neighbor: abs({}) > 1", mvi, mvj, mvi),
        }
        match mvj {
            -1 => if i == 0 { i = self.hgt - 1; } else { i -= 1;},
            1 => if i == self.hgt - 1 { i = 0 } else { i += 1; },
            0 => (),
            _ => panic!("({}, {}) is not a neighbor: abs({}) > 1", mvi, mvj, mvi),
        }
        [i, j]
    }

    fn topple(&mut self, i: usize, j: usize) {
        let fall = self.field[[i, j]].hgt / 4;
        self.field[[i, j]].scheduled = false;
        if fall > 0 {
            self.field[[i, j]].hgt -= fall * 4;
            for (mvi, mvj) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                if self.is_valid_move(i, j, *mvi, *mvj) {
                    let [ni, nj] = self.index_move(i, j, *mvi, *mvj);
                    self.field[[ni, nj]].hgt += fall;
                    if self.is_unstable(ni, nj) {
                        self.schedule.push_back((ni, nj));
                        self.field[[ni, nj]].scheduled = true;
                    }
                }
            }
        }
    }

    pub fn stabilize(&mut self) {
        loop {
            match self.schedule.pop_front() {
                None => break,
                Some((i, j)) => {
                    self.topple(i, j);
                    self.cnt += 1;
                }
            }
        }
    }

    fn is_unstable(&self, i: usize, j: usize) -> bool {
        let g = &self.field[[i, j]];
        g.hgt > 3 && !g.scheduled
    }

    pub fn render(&mut self, cfg: &mut crate::Config) {
        let name = cfg.frame();
        self.field.render(&name);

        eprintln!("Done rendering frame {} : workload {}", name, self.cnt);
        self.cnt = 0;
    }

    pub fn add(&mut self, i: usize, j: usize, amount: usize) {
        self.field[[i, j]].hgt += amount;
        if self.is_unstable(i, j) {
            self.schedule.push_back((i, j));
            self.field[[i, j]].scheduled = true;
        }
    }
}
