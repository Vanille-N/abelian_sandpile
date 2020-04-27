use std::fs::File;
use std::io::{BufWriter, Write};
use std::collections::VecDeque;

#[derive(Clone)]
struct Grain {
    hgt: usize,
    scheduled: bool,
}

pub struct Sandpile {
    field: Vec<Vec<Grain>>,
    hgt: usize,
    wth: usize,
    schedule: VecDeque<(usize, usize)>,
    idx: usize,
    name: String,
    cnt: usize,
}


impl Sandpile {
    pub fn new(i: usize, j: usize, name: String) -> Self {
        Sandpile {
            field: vec![vec![Grain { hgt: 0, scheduled: false }; j]; i],
            hgt: i,
            wth: j,
            schedule: VecDeque::new(),
            idx: 0,
            name,
            cnt: 0,
        }
    }

    fn topple(&mut self, i: usize, j: usize) {
        let fall = self.field[i][j].hgt / 4;
        self.field[i][j].scheduled = false;
        if fall > 0 {
            self.field[i][j].hgt -= fall * 4;
            for (mvi, mvj) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                let (ni, nj) = (
                    (i as isize + mvi) as usize,
                    (j as isize + mvj) as usize
                );
                if self.is_valid_idx(ni, nj) {
                    self.field[ni][nj].hgt += fall;
                    if self.is_unstable(ni, nj) {
                        self.schedule.push_back((ni, nj));
                        self.field[ni][nj].scheduled = true;
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

    fn is_valid_idx(&self, i: usize, j: usize) -> bool {
        i < self.hgt && j < self.wth
    }

    fn is_unstable(&self, i: usize, j: usize) -> bool {
        let g = &self.field[i][j];
        g.hgt > 3 && !g.scheduled
    }

    pub fn render(&mut self) {
        let mut f = BufWriter::new(File::create(&format!(".sandpile_{}/.out{}.ppm", self.name, lpad(self.idx, 5)))
            .unwrap());
        write!(f, "P3\n{} {}\n25\n", self.wth, self.hgt).unwrap();
        for line in &self.field {
            for g in line {
                let (r, g, b) = {
                    match g.hgt {
                        0 => (0, 0, 0),
                        1 => (13, 4, 0),
                        2 => (25, 9, 0),
                        3 => (25, 20, 0),
                        _ => (0, 0, 0),
                    }
                };
                write!(f, "{} {} {} ", r, g, b).unwrap();
            }
        }
        f.flush().unwrap();
        eprintln!("Done rendering frame number {} : workload {}", self.idx, self.cnt);
        self.idx += 1;
        self.cnt = 0;
    }

    pub fn add(&mut self, i: usize, j: usize, amount: usize) {
        self.field[i][j].hgt += amount;
        if self.is_unstable(i, j) {
            self.schedule.push_back((i, j));
            self.field[i][j].scheduled = true;
        }
    }
}


fn lpad(s: usize, len: usize) -> String {
    let s = format!("{}", s);
    let l = s.len();
    format!("{}{}", "0".repeat(len - l), s)
}
