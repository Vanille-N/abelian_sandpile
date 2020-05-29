use rand::Rng;

use crate::canvas::*;

/// A cell in Brian's brain can have any of three states
#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    Ready,
    Firing,
    Dying,
}

impl State {
    /// Ready -> Firing -> Dying -> loop
    pub fn next(self) -> State {
        match self {
            State::Ready => State::Firing,
            State::Firing => State::Dying,
            State::Dying => State::Ready,
        }
    }
}

/// A single cell in the Brian's Brain automaton
#[derive(Clone, Copy)]
struct Neuron {
    curr: State,
    /// Changing the state has to be done after all cells have been checked
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

/// A collection of Neurons
pub struct Brain {
    field: Canvas<Neuron>,
    hgt: usize,
    wth: usize,
    fired: usize,
}

impl Brain {
    /// A new Brain is constructed from only its dimensions
    pub fn new(hgt: usize, wth: usize) -> Self {
        Self {
            field: Canvas::new(hgt, wth, Neuron::new()),
            hgt,
            wth,
            fired: 0,
        }
    }

    /// Randomly fire some cells (all over the canvas).
    /// p is the probability for any cell of being fired
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

    /// Randomly fire some cells (only in a small area near the center).
    /// f is the fraction (both vertically and horizontally) of the canvas
    /// on which to fire neurons.
    /// p is the probability of any neuron of being fired
    pub fn init_cluster(&mut self, f: f64, p: f64) {
        let mut rng = rand::thread_rng();
        let lo = |n| (n as f64 * (1. - f) / 2.).floor() as usize;
        let hi = |n| (n as f64 * (1. + f) / 2.).floor() as usize;
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

    /// Actualize current state with previously calculated next state
    pub fn update(&mut self) {
        for i in 0..self.hgt {
            for j in 0..self.wth {
                self.field[[i, j]].update();
            }
        }
    }

    /// 2D Array access with automatic looping around the edges.
    /// Only works for direct neighbors (Moore neighborhood)
    fn index_move(&self, i: usize, j: usize, mvi: isize, mvj: isize) -> [usize; 2] {
        let (mut i, mut j) = (i, j);
        match mvi {
            -1 => {
                if i == 0 {
                    i = self.hgt - 1;
                } else {
                    i -= 1;
                }
            }
            1 => {
                if i == self.hgt - 1 {
                    i = 0
                } else {
                    i += 1;
                }
            }
            0 => (),
            _ => panic!("({}, {}) is not a neighbor: abs({}) > 1", mvi, mvj, mvi),
        }
        match mvj {
            -1 => {
                if j == 0 {
                    j = self.hgt - 1;
                } else {
                    j -= 1;
                }
            }
            1 => {
                if j == self.hgt - 1 {
                    j = 0
                } else {
                    j += 1;
                }
            }
            0 => (),
            _ => panic!("({}, {}) is not a neighbor: abs({}) > 1", mvi, mvj, mvj),
        }
        [i, j]
    }

    // Count neighbors (with looping around the edges) who are fired
    fn count_neigh(&self, i: usize, j: usize) -> usize {
        let mut res = 0;
        if self.field[self.index_move(i, j, -1, 0)].is_firing() {
            res += 1;
        }
        if self.field[self.index_move(i, j, -1, -1)].is_firing() {
            res += 1;
        }
        if self.field[self.index_move(i, j, -1, 1)].is_firing() {
            res += 1;
        }
        if self.field[self.index_move(i, j, 1, 0)].is_firing() {
            res += 1;
        }
        if self.field[self.index_move(i, j, 1, -1)].is_firing() {
            res += 1;
        }
        if self.field[self.index_move(i, j, 1, 1)].is_firing() {
            res += 1;
        }
        if self.field[self.index_move(i, j, 0, -1)].is_firing() {
            res += 1;
        }
        if self.field[self.index_move(i, j, 0, 1)].is_firing() {
            res += 1;
        }
        res
    }

    /// Rules of Brian's Brain for how a neuron changes state
    /// ```
    /// State::Firing -> State::Dying
    /// State::Dying -> State::Ready
    /// State::Ready -> State::Firing iff 2 firing neighbors
    /// ```
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

    /// Output current state of the canvas as an image
    pub fn render(&mut self, cfg: &mut crate::Config) {
        let name = cfg.frame();
        self.field.render(&name);

        eprint!("\rDone generation {} : {} fired", name, self.fired);
    }
}

impl Neuron {
    /// All neurons are initialized ready to fire
    pub fn new() -> Self {
        Self {
            curr: State::Ready,
            succ: State::Ready,
        }
    }

    /// Ready -> Firing -> Dying -> loop
    pub fn next(&mut self) {
        self.succ = self.curr.next();
    }

    /// Set current state to calculated next state
    pub fn update(&mut self) {
        self.curr = self.succ;
    }

    pub fn is_firing(self) -> bool {
        self.curr == State::Firing
    }
}
