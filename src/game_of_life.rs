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

pub struct Colony {
    field: Canvas<Cell>,
    hgt: usize,
    wth: usize,
    cnt: usize,
    born: usize,
    dead: usize,
}

impl Colony {
    pub fn new(hgt: usize, wth: usize) -> Self {
        Self {
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
                    self.field[[i as isize, j as isize]].birth();
                }
            }
        }
        self.update();
    }

    pub fn init_cluster(&mut self, f: f64, p: f64) {
        let mut rng = rand::thread_rng();
        let lo = |n| {
            (n as f64 * (1. - f) / 2.).floor() as isize
        };
        let hi = |n| {
            (n as f64 * (1. + f) / 2.).floor() as isize
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

    fn count_neigh(&self, i: isize, j: isize) -> u8 {
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
                let neigh = self.count_neigh(i as isize, j as isize);
                let cell = &mut self.field[[i as isize, j as isize]];
                if cell.is_alive() {
                    match neigh {
                        2 | 3 => (),
                        _ => {
                            cell.kill();
                            self.dead += 1;
                        }
                    }
                } else {
                    match neigh {
                        3 => {
                            cell.birth();
                            self.born += 1;
                        }
                        _ => (),
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
