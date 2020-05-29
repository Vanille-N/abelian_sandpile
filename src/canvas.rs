use std::fs::File;
use std::io::{BufWriter, Write};

/// RGB color
/// Each component should be in the range [0, 25]
pub type Color = (u8, u8, u8);

/// Indicates that a object of a given type can be converted to a color.
pub trait Colorize<T = Self>: Copy {
    fn color(&self) -> Color;
}

/// A canvas is a 2D array of objects that we know how to convert to colors
pub struct Canvas<T: Colorize> {
    hgt: usize,
    wth: usize,
    tab: Vec<Vec<T>>,
}

impl<T: Colorize> Canvas<T> {
    /// Fill with a default value
    /// This is why the Copy trait is required
    pub fn new(hgt: usize, wth: usize, init: T) -> Self {
        Self {
            hgt,
            wth,
            tab: vec![vec![init; wth]; hgt],
        }
    }

    /// Output the current state of the canvas to a file
    pub fn render(&self, name: &str) {
        let mut f = BufWriter::new(File::create(name).unwrap());
        write!(f, "P3\n{} {}\n25\n", self.wth, self.hgt).unwrap();
        for line in &self.tab {
            for g in line {
                let (r, g, b) = g.color();
                write!(f, "{} {} {} ", r, g, b).unwrap();
            }
        }
        f.flush().unwrap();
    }

    /// Access array cells with wrapping
    pub fn mod_idx(&mut self, i: isize, j: isize) -> &mut T {
        &mut self.tab[mod_idx(i, self.hgt)][mod_idx(j, self.wth)]
    }
}

impl<T: Colorize> std::ops::Index<[usize; 2]> for Canvas<T> {
    type Output = T;

    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        &self.tab[idx[0]][idx[1]]
    }
}

impl<T: Colorize> std::ops::IndexMut<[usize; 2]> for Canvas<T> {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut T {
        &mut self.tab[idx[0]][idx[1]]
    }
}

/// Wrap i to the range [0, n)
fn mod_idx(i: isize, n: usize) -> usize {
    if i >= 0 {
        (i % n as isize) as usize
    } else {
        let p = n / (-i as usize);
        let i = (i as usize) + p * n;
        i % n
    }
}
