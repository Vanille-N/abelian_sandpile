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

    fn topple(&mut self, i: isize, j: isize) {
        let fall = self.field[[i, j]].hgt / 4;
        self.field[[i, j]].scheduled = false;
        if fall > 0 {
            self.field[[i, j]].hgt -= fall * 4;
            for (mvi, mvj) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                let (ni, nj) = ((i + mvi), (j + mvj));
                if self.is_valid_idx(ni, nj) {
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
