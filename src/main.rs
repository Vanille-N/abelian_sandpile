use std::fs;
use std::process::Command;

mod sandpile;
mod canvas;

use sandpile::*;

fn main() {
    let name = String::from("multiple");
    let algo = Automaton::Sandpile;
    let mut cfg = Config::make(algo, name);

    cfg.prepare();
    render(&mut cfg);
    cfg.build();
}

fn render(cfg: &mut Config) {
    match cfg.algo {
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
    }
}

pub struct Config {
    algo: Automaton,
    name: String,
    idx: usize,
}

impl Config {
    pub fn make(algo: Automaton, name: String) -> Self {
        Self {
            algo,
            name,
            idx: 0,
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
                    "-crf", "15",
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
}

impl Automaton {
    pub fn str(&self) -> String {
        String::from(match self {
            Automaton::Sandpile => "sand",
        })
    }
}
