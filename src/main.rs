#![allow(dead_code)]

use std::fs;
use std::process::Command;

mod brain;
mod canvas;
mod langton;
mod lifelike;
mod sandpile;

use brain::*;
use langton::*;
use lifelike::*;
use sandpile::*;

fn main() {
    let name = String::from("gun");
    let algo = Automaton::LifeLike(String::from(LIFE));
    let mut cfg = Config::new(algo, name, 25);

    cfg.prepare();
    render(&mut cfg);
    cfg.build();
}

fn render(cfg: &mut Config) {
    match &cfg.algo {
        Automaton::Sandpile => {
            let mut pile = Sandpile::new(201, 201);
            for _ in 0..3000 {
                pile.render(cfg);
                pile.add(100, 100, 10);
                pile.add(140, 150, 10);
                pile.add(150, 40, 10);
                pile.add(70, 50, 10);
                pile.add(20, 150, 10);
                pile.stabilize();
            }
        }
        Automaton::LifeLike(rules) => {
            let mut game = LifeLike::new(200, 300, &rules);
            game.add_from_file("data/unknown.txt", 5, 150, Rotate::None);
            for _ in 0..5000 {
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
        Automaton::Langton => {
            let mut anthill = Langton::new(1000, 1000);
            for _ in 0..100 {
                anthill.add_rand_ant();
            }
            for _ in 0..1000 {
                anthill.multi(100);
                anthill.render(cfg);
            }
        }
    }
}

pub struct Config {
    algo: Automaton,
    name: String,
    idx: usize,
    framerate: usize,
}

impl Config {
    pub fn new(algo: Automaton, name: String, framerate: usize) -> Self {
        Self {
            algo,
            name,
            idx: 0,
            framerate,
        }
    }

    fn dir(&self) -> String {
        format!(".{}_{}", self.algo.str(), self.name)
    }

    fn file(&self) -> String {
        format!("{}_{}.avi", self.algo.str(), self.name)
    }

    fn frame(&mut self) -> String {
        let idx = self.idx;
        self.idx += 1;
        format!("{}/out-{}.ppm", self.dir(), Self::lpad(idx, 5))
    }

    pub fn prepare(&self) {
        let _ = Command::new("rm")
            .arg("-r")
            .arg(&self.file())
            .status()
            .expect("Cleanup aborted");
        let _ = Command::new("rm")
            .arg("-r")
            .arg(&self.dir())
            .status()
            .expect("Cleanup aborted");
        fs::create_dir(self.dir()).expect(&format!("could not create directory {}", self.dir()));
    }

    pub fn build(&self) {
        eprintln!("All calculations done");
        let _ = Command::new("ffmpeg")
            .args(&[
                "-pattern_type",
                "glob",                 // find all frames according to glob pattern
                "-framerate",
                "25",                   // 25 FPS
                "-i",
                &format!("{}/*.ppm", self.dir()),
                "-vf",
                "scale=1000:-1",         // rescale to 1000px (keep aspect ratio)
                "-sws_flags",
                "neighbor",             // no interpolation
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

    fn lpad(s: usize, len: usize) -> String {
        let s = format!("{}", s);
        let l = s.len();
        format!("{}{}", "0".repeat(len - l), s)
    }
}

pub enum Automaton {
    Sandpile,
    LifeLike(String),
    Brain,
    Langton,
}

impl Automaton {
    pub fn str(&self) -> String {
        match self {
            Automaton::Sandpile => String::from("sand"),
            Automaton::LifeLike(rules) => format!("life-{}", rules),
            Automaton::Brain => String::from("brain"),
            Automaton::Langton => String::from("ant"),
        }
    }
}
