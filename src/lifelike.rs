extern crate rand;
use rand::Rng;

use crate::canvas::*;

#[derive(Clone, Copy)]
struct Cell {
    curr: bool,
    succ: bool,
}

impl Colorize for Cell {
    fn color(&self) -> Color {
        if self.curr {
            (25, 25, 25)
        } else {
            (0, 0, 0)
        }
    }
}

pub struct LifeLike {
    rules: Rules,
    field: Canvas<Cell>,
    hgt: usize,
    wth: usize,
    cnt: usize,
    born: usize,
    dead: usize,
}

impl LifeLike {
    pub fn new(hgt: usize, wth: usize, rules: &str) -> Self {
        Self {
            rules: Rules::new(rules),
            field: Canvas::new(hgt, wth, Cell::new()),
            hgt,
            wth,
            cnt: 0,
            born: 0,
            dead: 0,
        }
    }

    pub fn init_rand(&mut self, p: f64) {
        let mut rng = rand::thread_rng();
        for i in 0..self.hgt {
            for j in 0..self.wth {
                if rng.gen::<f64>() < p {
                    self.cnt += 1;
                    self.field[[i, j]].birth();
                }
            }
        }
        self.update();
    }

    pub fn init_cluster(&mut self, f: f64, p: f64) {
        let mut rng = rand::thread_rng();
        let lo = |n| {
            (n as f64 * (1. - f) / 2.).floor() as usize
        };
        let hi = |n| {
            (n as f64 * (1. + f) / 2.).floor() as usize
        };
        for i in lo(self.hgt)..hi(self.hgt) {
            for j in lo(self.wth)..hi(self.wth) {
                if rng.gen::<f64>() < p {
                    self.cnt += 1;
                    self.field[[i, j]].birth();
                }
            }
        }
        self.update();
    }

    pub fn update(&mut self) {
        for i in 0..self.hgt {
            for j in 0..self.wth {
                self.field[[i as isize, j as isize]].update();
            }
        }
    }

    fn count_neigh(&self, i: isize, j: isize) -> usize {
        let mut res = 0;
        if self.field[[i-1, j]].is_alive() { res += 1; }
        if self.field[[i-1, j-1]].is_alive() { res += 1; }
        if self.field[[i-1, j+1]].is_alive() { res += 1; }
        if self.field[[i+1, j]].is_alive() { res += 1; }
        if self.field[[i+1, j-1]].is_alive() { res += 1; }
        if self.field[[i+1, j+1]].is_alive() { res += 1; }
        if self.field[[i, j-1]].is_alive() { res += 1; }
        if self.field[[i, j+1]].is_alive() { res += 1; }
        res
    }

    pub fn next(&mut self) {
        self.born = 0;
        self.dead = 0;
        for i in 0..self.hgt {
            for j in 0..self.wth {
                let neigh = self.count_neigh(i, j);
                let cell = &mut self.field[[i, j]];
                if cell.is_alive() {
                    if !self.rules.s[neigh] {
                        cell.kill();
                        self.dead += 1;
                    }
                } else {
                    if self.rules.b[neigh] {
                        cell.birth();
                        self.born += 1;
                    }
                }
            }
        }
        self.cnt += self.born;
        self.cnt -= self.dead;
        self.update();
    }

    pub fn render(&mut self, cfg: &mut crate::Config) {
        let name = cfg.frame();
        self.field.render(&name);

        eprintln!("Done generation {} : {} alive (+{} ; -{})", name, self.cnt, self.born, self.dead);
    }
}


impl Cell {
    pub fn new() -> Self {
        Self {
            curr: false,
            succ: false,
        }
    }

    pub fn birth(&mut self) {
        self.succ = true;
    }

    pub fn kill(&mut self) {
        self.succ = false;
    }

    pub fn update(&mut self) {
        if self.succ {
            if !self.curr {
                self.curr = true;
            }
        } else {
            if self.curr {
                self.curr = false;
            }
        }
    }

    pub fn is_alive(&self) -> bool {
        self.curr
    }
}

#[derive(Clone, Copy)]
struct Rules {
    b: [bool; 9],
    s: [bool; 9],
}

impl Rules {
    pub fn new(s: &str) -> Self {
        let mut r = Rules { b: [false; 9], s: [false; 9] };
        let v: Vec<_> = s.split("-").collect();
        let (b, s) = (v[0], v[1]);
        for c in b.chars() {
            r.b[c.to_digit(10).unwrap() as usize] = true;
        }
        for c in s.chars() {
            r.s[c.to_digit(10).unwrap() as usize] = true;
        }
        r
    }
}

pub const LIFE: &str = "3-23";
pub const REPLICATOR: &str = "1357-1357";
pub const SEEDS: &str = "2-";
pub const NODEATH: &str = "3-012345678";
pub const LIFE34: &str = "34-34";
pub const DIAMOEBA: &str = "35678-5678";
pub const X22: &str = "36-125";
pub const HIGHLIFE: &str = "36-23";
pub const DAYNIGHT: &str = "3678-34678";
pub const MORLEY: &str = "368-245";
pub const ANNEAL: &str = "4678-35678";
