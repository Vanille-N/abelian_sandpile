#![allow(dead_code)]

use std::fs;
use std::process::Command;

mod brain;
mod canvas;
mod lifelike;
mod sandpile;
mod turmite;

use brain::*;
use lifelike::*;
use sandpile::*;
use turmite::*;

fn main() {
    let name = String::from("breeder");
    let algo = Automaton::LifeLike(LIFE);
    let mut cfg = Config::new(algo, name, 25);

    cfg.prepare();
    render(&mut cfg);
    cfg.build();
}

fn render(cfg: &mut Config) {
    match &cfg.algo {
        Automaton::Sandpile => {
            let mut pile = Sandpile::new(201, 201);
            for _ in 0..1000 {
                pile.render(cfg);
                pile.add(100, 100, 5);
                pile.add(110, 110, 10);
                pile.add(120, 120, 20);
                pile.add(90, 90, 10);
                pile.add(80, 80, 20);
                pile.stabilize();
            }
        }
        Automaton::LifeLike(rules) => {
            let mut game = LifeLike::new(1000, 2000, &rules);
            game.add_from_file("data/patterns/p69060p5h2v0gun.rle", 0, 0, T_NONE);
            for _ in 0..2000 {
                game.render(cfg);
                game.next();
            }
        }
        Automaton::Brain => {
            let mut brain = Brain::new(300, 400);
            brain.init_cluster(0.05, 0.3);
            for _ in 0..5000 {
                brain.render(cfg);
                brain.next();
            }
        }
        Automaton::Turmite(rules) => {
            let mut mound = Mound::new(900, 900, rules);
            for _ in 0..50 {
                mound.add_rand([449, 452], [449, 452], None);
            }
            for _ in 0..2000 {
                mound.multi(50);
                mound.render(cfg);
            }
        }
    }
}

/// Stores all the relevant information needed to initialize an automaton
pub struct Config<'a> {
    algo: Automaton<'a>,
    name: String,
    idx: usize,
    framerate: usize,
}

impl<'a> Config<'a> {
    /// Create a new simulation
    pub fn new(algo: Automaton<'a>, name: String, framerate: usize) -> Self {
        Self {
            algo,
            name,
            idx: 0,
            framerate,
        }
    }

    /// File directory: `.{type}_{name}/`
    fn dir(&self) -> String {
        format!(".{}_{}", self.algo.str(), self.name)
    }

    /// Final product: `{type}_{name}.avi`
    fn file(&self) -> String {
        format!("{}_{}.avi", self.algo.str(), self.name)
    }

    /// Single state filename: `.{type}_{name}/out-{num}.ppm`
    /// where num is automatically incremented and left-padded with 0s.
    fn frame(&mut self) -> String {
        let idx = self.idx;
        self.idx += 1;
        format!("{}/out-{}.ppm", self.dir(), Self::lpad(idx, 5))
    }

    /// Cleanup directory: remove avi target if it already exists and
    /// remove directory of ppm files in case it was not properly
    /// deleted during the previous execution.
    pub fn prepare(&self) {
        let _ = Command::new("rm")
            .arg(&self.file())
            .status()
            .expect("Cleanup aborted");
        let _ = Command::new("rm")
            .arg("-r")
            .arg(&self.dir())
            .status()
            .expect("Cleanup aborted");
        fs::create_dir(self.dir())
            .unwrap_or_else(|_| panic!("could not create directory {}", self.dir()));
    }

    /// Concatenate all ppm files into a single video,
    /// then cleanup temporary files.
    pub fn build(&self) {
        eprintln!("All calculations done");
        let _ = Command::new("ffmpeg")
            .args(&[
                "-pattern_type",
                "glob", // find all frames according to glob pattern
                "-framerate",
                "25", // 25 FPS
                "-i",
                &format!("{}/*.ppm", self.dir()),
                "-vf",
                "scale=1000:-1", // rescale to 1000px (keep aspect ratio)
                "-sws_flags",
                "neighbor", // no interpolation
                "-vcodec",
                "libx264",
                "-crf",
                &format!("{}", self.framerate),
                &self.file(),
            ])
            .status()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
        let _ = Command::new("rm")
            .arg("-r")
            .arg(&self.dir())
            .status()
            .expect("Cleanup aborted");
    }

    /// Left-pad usize with zeros
    fn lpad(s: usize, len: usize) -> String {
        let s = format!("{}", s);
        let l = s.len();
        format!("{}{}", "0".repeat(len - l), s)
    }
}

/// Types of automata available
pub enum Automaton<'a> {
    Sandpile,
    LifeLike(&'a str),
    Brain,
    Turmite(Rules<'a>),
}

impl Automaton<'_> {
    /// Associate automaton type to its name (used in the filenames)
    pub fn str(&self) -> String {
        match self {
            Automaton::Sandpile => String::from("sand"),
            Automaton::LifeLike(rules) => format!("life-{}", rules),
            Automaton::Brain => String::from("brain"),
            Automaton::Turmite(_) => String::from("turmite"),
        }
    }
}
