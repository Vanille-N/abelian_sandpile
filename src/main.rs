use std::fs;
use std::process::Command;

mod sandpile;
mod canvas;
mod lifelike;
mod brain;

use sandpile::*;
use lifelike::*;
use brain::*;

fn main() {
    let name = String::from("rand");
    let algo = Automaton::LifeLike(DAYNIGHT.to_string());
    let mut cfg = Config::new(algo, name, 25);

    cfg.prepare();
    render(&mut cfg);
    cfg.build();
}

fn render(cfg: &mut Config) {
    match &cfg.algo {
        Automaton::Sandpile => {
            let mut pile = Sandpile::new(201, 201);
            for i in 0..3000 {
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
            let mut game = LifeLike::new(300, 500, &rules);
            game.init_cluster(0.4, 0.3);
            for i in 0..3000 {
                game.render(cfg);
                game.next();
            }
        }
        Automaton::Brain => {
            let mut brain = Brain::new(300, 400);
            brain.init_cluster(0.05, 0.3);
            for i in 0..5000 {
                brain.render(cfg);
                brain.next();
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
        fs::create_dir(self.dir())
            .expect(&format!("could not create directory {}", self.dir()));
    }

    pub fn build(&self) {
        eprintln!("All calculations done");
        let _ = Command::new("ffmpeg")
            .args(&["-pattern_type", "glob",
                    "-framerate", "25",
                    "-i", &format!("{}/*.ppm", self.dir()),
                    "-vcodec", "libx264",
                    "-crf", &format!("{}", self.framerate),
                    &self.file()])
            .status()
            .unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
            });
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
}

impl Automaton {
    pub fn str(&self) -> String {
        match self {
            Automaton::Sandpile => String::from("sand"),
            Automaton::LifeLike(rules) => format!("life-{}", rules),
            Automaton::Brain => String::from("brain"),
        }
    }
}
