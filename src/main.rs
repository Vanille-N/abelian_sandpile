use std::fs;
use std::process::Command;

mod sandpile;

use sandpile::*;

fn main() {
    let name = String::from("multiple");
    let algo = String::from("sandpile");

    let _ = Command::new("rm")
        .arg("-r")
        .arg(&format!("{}_{}.avi", algo, name))
        .status()
        .expect("Cleanup aborted");
    fs::create_dir(format!(".{}_{}", algo, name))
        .expect(&format!("could not create directory {}_{}", algo, name));
    render(name.clone());
    eprintln!("All calculations done");
    let _ = Command::new("ffmpeg")
        .args(&["-pattern_type", "glob",
                "-framerate", "25",
                "-i", &format!(".{}_{}/.*.ppm", algo, name),
                "-vcodec", "libx264",
                "-crf", "15",
                &format!("{}_{}.avi", algo, name)])
        .status()
        .unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
    let _ = Command::new("rm")
        .arg("-r")
        .arg(&format!(".{}_{}/", algo, name))
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
