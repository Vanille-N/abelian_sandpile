use crate::canvas::*;
use rand::Rng;

type Mark = usize;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Turn {
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub enum Dir {
    N,
    S,
    W,
    E,
}

pub type Rules<'a> = &'a [&'a [(Mark, Turn)]];

type Pos = [usize; 2];

#[derive(Clone, Copy)]
struct Ant {
    pos: Pos,
    dir: Dir,
    rules: usize,
}

pub struct Anthill<'a> {
    hgt: usize,
    wth: usize,
    map: Rules<'a>,
    field: Canvas<Mark>,
    ants: Vec<Ant>,
    cnt: usize,
}

impl Dir {
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

impl<'a> Anthill<'a> {
    pub fn new(hgt: usize, wth: usize, rules: Rules<'a>) -> Self {
        Self {
            hgt,
            wth,
            map: rules,
            field: Canvas::new(hgt, wth, 0),
            ants: Vec::new(),
            cnt: 0,
        }
    }

    pub fn add(&mut self, pos: Pos, dir: Dir, rules: usize) {
        self.ants.push(Ant { pos, dir, rules });
    }

    pub fn add_rand(
        &mut self,
        [imin, imax]: [usize; 2],
        [jmin, jmax]: [usize; 2],
        rules_rng: Option<usize>,
    ) {
        let mut rng = rand::thread_rng();
        self.ants.push(Ant {
            pos: [rng.gen_range(imin, imax), rng.gen_range(jmin, jmax)],
            dir: Dir::from(rng.gen_range(0, 4)),
            rules: match rules_rng {
                Some(n) => n,
                None => rng.gen_range(0, self.map.len()),
            },
        });
    }

    pub fn next(&mut self) {
        for ant in &mut self.ants {
            let (m, t) = self.map[ant.rules][self.field[ant.pos]];
            ant.turn(t);
            self.field[ant.pos] = m;
            ant.mv(self.hgt, self.wth);
        }
        self.cnt += 1;
    }

    pub fn multi(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }

    pub fn render(&mut self, cfg: &mut crate::Config) {
        let name = cfg.frame();
        self.field.render(&name);

        eprint!("\rDone frame {} ({}'th movement)", name, self.cnt);
    }
}

impl Ant {
    pub fn turn(&mut self, t: Turn) {
        self.dir = self.dir.turn(t);
    }

    pub fn mv(&mut self, imax: usize, jmax: usize) {
        self.pos = mv(self.pos, self.dir, imax, jmax);
    }
}

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
