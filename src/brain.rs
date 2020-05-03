extern crate rand;
use rand::Rng;

use crate::canvas::*;

#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    Ready,
    Firing,
    Dying,
}

#[derive(Clone, Copy)]
struct Neuron {
    curr: State,
    succ: State,
}

impl Colorize for Neuron {
    fn color(&self) -> Color {
        match self.curr {
            State::Ready => (0, 0, 0),
            State::Firing => (25, 25, 25),
            State::Dying => (0, 0, 25),
        }
    }
}

pub struct Brain {
    field: Canvas<Neuron>,
    hgt: usize,
    wth: usize,
    fired: usize,
}

impl Brain {
    pub fn new(hgt: usize, wth: usize) -> Self {
        Self {
            field: Canvas::new(hgt, wth, Neuron::new()),
            hgt,
            wth,
            fired: 0,
        }
    }

    pub fn init_rand(&mut self, p: f64) {
        let mut rng = rand::thread_rng();
        for i in 0..self.hgt {
            for j in 0..self.wth {
                if rng.gen::<f64>() < p {
                    self.fired += 1;
                    self.field[[i, j]].fire();
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
                    self.fired += 1;
                    self.field[[i, j]].fire();
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
        if self.field[[i-1, j]].is_firing() { res += 1; }
        if self.field[[i-1, j-1]].is_firing() { res += 1; }
        if self.field[[i-1, j+1]].is_firing() { res += 1; }
        if self.field[[i+1, j]].is_firing() { res += 1; }
        if self.field[[i+1, j-1]].is_firing() { res += 1; }
        if self.field[[i+1, j+1]].is_firing() { res += 1; }
        if self.field[[i, j-1]].is_firing() { res += 1; }
        if self.field[[i, j+1]].is_firing() { res += 1; }
        res
    }

    pub fn next(&mut self) {
        self.fired = 0;
        for i in 0..self.hgt {
            for j in 0..self.wth {
                match self.field[[i, j]].curr {
                    State::Firing => {
                        self.field[[i, j]].refactor();
                    }
                    State::Dying => {
                        self.field[[i, j]].kill();
                    }
                    State::Ready => {
                        if self.count_neigh(i, j) == 2 {
                            self.field[[i, j]].fire();
                            self.fired += 1;
                        }
                    }
                }

            }
        }
        self.update();
    }

    pub fn render(&mut self, cfg: &mut crate::Config) {
        let name = cfg.frame();
        self.field.render(&name);

        eprintln!("Done generation {} : {} fired", name, self.fired);
    }
}


impl Neuron {
    pub fn new() -> Self {
        Self {
            curr: State::Ready,
            succ: State::Ready,
        }
    }

    pub fn fire(&mut self) {
        self.succ = State::Firing;
    }

    pub fn kill(&mut self) {
        self.succ = State::Ready;
    }

    pub fn refactor(&mut self) {
        self.succ = State::Dying;
    }

    pub fn update(&mut self) {
        self.curr = self.succ;
    }

    pub fn is_firing(&self) -> bool {
        self.curr == State::Firing
    }
}
