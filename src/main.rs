use std::fs;
use std::process::Command;

mod sandpile;

use sandpile::*;

fn main() {
    let name = String::from("multiple");
    let _ = Command::new("rm")
        .arg("-r")
        .arg(&format!("{}.avi", name))
        .status()
        .expect("Cleanup aborted");
    fs::create_dir(format!("._{}", name))
        .expect(&format!("could not create directory {}", name));
    render(name.clone());
    eprintln!("All calculations done");
    let _ = Command::new("ffmpeg")
        .args(&["-pattern_type", "glob",
                "-framerate", "25",
                "-i", &format!("._{}/.*.ppm", name),
                "-vcodec", "libx264",
                "-crf", "15",
                &format!("{}.avi", name)])
        .status()
        .unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
    let _ = Command::new("rm")
        .arg("-r")
        .arg(&format!("._{}/", name))
        .status()
        .expect("Cleanup aborted");
}

fn render(name: String) {
    let mut pile = Sandpile::new(201, 201, name);
    for i in 0..3000 {
        pile.render();
        pile.add(100, 100, 10);
        pile.add(140, 150, 10);
        pile.add(150, 40, 10);
        pile.add(70, 50, 10);
        pile.add(20, 150, 10);
        pile.stabilize();
    }
}
