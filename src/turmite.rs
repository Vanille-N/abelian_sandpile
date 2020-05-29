use crate::canvas::*;
use rand::Rng;

/// Trace left by the turmites
type Mark = usize;

/// Change orientation
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Turn {
    Left,
    Right,
}

/// Orientation of a turmite
#[derive(Clone, Copy)]
pub enum Dir {
    N,
    S,
    W,
    E,
}

/// Transition table for how to move the turmite
pub type Rules<'a> = &'a [&'a [(Mark, Turn)]];

/// Position on the canvas of a turmite
type Pos = [usize; 2];

/// A single turmite
#[derive(Clone, Copy)]
struct Turmite {
    pos: Pos,
    dir: Dir,
    rules: usize,
}

/// A collection of turmites, along with their environment
pub struct Mound<'a> {
    hgt: usize,
    wth: usize,
    map: Rules<'a>,
    field: Canvas<Mark>,
    turmites: Vec<Turmite>,
    cnt: usize,
}

impl Dir {
    /// Update new direction
    fn turn(self, t: Turn) -> Dir {
        match self {
            Dir::N => {
                if t == Turn::Left {
                    Dir::W
                } else {
                    Dir::E
                }
            }
            Dir::S => {
                if t == Turn::Left {
                    Dir::E
                } else {
                    Dir::W
                }
            }
            Dir::E => {
                if t == Turn::Left {
                    Dir::N
                } else {
                    Dir::S
                }
            }
            Dir::W => {
                if t == Turn::Left {
                    Dir::S
                } else {
                    Dir::N
                }
            }
        }
    }

    /// Map integers to directions to enable random generation
    fn from(i: usize) -> Self {
        match i {
            0 => Dir::N,
            1 => Dir::E,
            2 => Dir::S,
            3 => Dir::W,
            _ => panic!("there are only 4 possible directions"),
        }
    }
}

impl Colorize for Mark {
    fn color(&self) -> Color {
        match self {
            0 => (0, 0, 0),
            1 => (9, 18, 2),
            2 => (17, 1, 25),
            3 => (9, 19, 25),
            4 => (25, 11, 0),
            5 => (0, 25, 10),
            6 => (25, 0, 0),
            _ => (25, 25, 25),
        }
    }
}

impl<'a> Mound<'a> {
    /// Create mound with no turmites and a blank environment
    pub fn new(hgt: usize, wth: usize, rules: Rules<'a>) -> Self {
        Self {
            hgt,
            wth,
            map: rules,
            field: Canvas::new(hgt, wth, 0),
            turmites: Vec::new(),
            cnt: 0,
        }
    }

    /// Add a turmite
    pub fn add(&mut self, pos: Pos, dir: Dir, rules: usize) {
        self.turmites.push(Turmite { pos, dir, rules });
    }

    /// Add a randomly generated turmite with restrictions on the range
    /// of positions and the possible orientations.
    pub fn add_rand(
        &mut self,
        [imin, imax]: [usize; 2],
        [jmin, jmax]: [usize; 2],
        rules_rng: Option<usize>,
    ) {
        let mut rng = rand::thread_rng();
        self.turmites.push(Turmite {
            pos: [rng.gen_range(imin, imax), rng.gen_range(jmin, jmax)],
            dir: Dir::from(rng.gen_range(0, 4)),
            rules: match rules_rng {
                Some(n) => n,
                None => rng.gen_range(0, self.map.len()),
            },
        });
    }

    /// Make all turmites by one step
    pub fn next(&mut self) {
        for turmite in &mut self.turmites {
            let (m, t) = self.map[turmite.rules][self.field[turmite.pos]];
            turmite.turn(t);
            self.field[turmite.pos] = m;
            turmite.mv(self.hgt, self.wth);
        }
        self.cnt += 1;
    }

    /// Make all turmites advance by many steps (each at their own turn)
    pub fn multi(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }

    /// Create image from current state
    pub fn render(&mut self, cfg: &mut crate::Config) {
        let name = cfg.frame();
        self.field.render(&name);

        eprint!("\rDone frame {} ({}'th movement)", name, self.cnt);
    }
}

impl Turmite {
    /// Change direction
    pub fn turn(&mut self, t: Turn) {
        self.dir = self.dir.turn(t);
    }

    /// Step forward by one (with wrapping around edges)
    pub fn mv(&mut self, imax: usize, jmax: usize) {
        self.pos = mv(self.pos, self.dir, imax, jmax);
    }
}

/// Calculate next position depending on direction (with wrapping)
fn mv(p: Pos, d: Dir, imax: usize, jmax: usize) -> Pos {
    match d {
        Dir::N => {
            if p[0] == 0 {
                [imax - 1, p[1]]
            } else {
                [p[0] - 1, p[1]]
            }
        }
        Dir::S => {
            if p[0] == imax - 1 {
                [0, p[1]]
            } else {
                [p[0] + 1, p[1]]
            }
        }
        Dir::W => {
            if p[1] == 0 {
                [p[0], jmax - 1]
            } else {
                [p[0], p[1] - 1]
            }
        }
        Dir::E => {
            if p[1] == jmax - 1 {
                [p[0], 0]
            } else {
                [p[0], p[1] + 1]
            }
        }
    }
}

pub const RULES_2: Rules = &[
    &[(1, Turn::Left), (0, Turn::Right)],
    &[(1, Turn::Right), (0, Turn::Left)],
];

pub const RULES_4: Rules = &[
    &[
        (1, Turn::Left),
        (2, Turn::Right),
        (3, Turn::Right),
        (0, Turn::Left),
    ],
    &[
        (1, Turn::Right),
        (2, Turn::Left),
        (3, Turn::Left),
        (0, Turn::Right),
    ],
];
