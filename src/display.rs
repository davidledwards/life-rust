use crate::{Point, Universe};

pub struct Display {
    xsize: u32,
    ysize: u32,
}

impl Display {
    pub fn new(bound: (u32, u32)) -> Display {
        let (xsize, ysize) = bound;
        Display { xsize, ysize }
    }

    pub fn render_basic(&self, u: &Universe) {
        let mut buf = String::new();
        for y in 0..self.ysize as i32 {
            for x in 0..self.xsize as i32 {
                let c = if u.alive.contains(&Point(x, y)) {
                    '#'
                } else {
                    ' '
                };
                buf.push(c);
            }
        }
        print!("\x1b[2J\x1b[0;0H{}", buf);
    }

    pub fn render_fancy(&self, u: &Universe) {
        let mut buf = String::new();
        for y in 0..self.ysize as i32 {
            for x in 0..self.xsize as i32 {
                let p = Point(x, y);
                let c = if u.born.contains(&p) {
                    '+'
                } else if u.alive.contains(&p) {
                    '#'
                } else if u.died.contains(&p) {
                    '-'
                } else {
                    ' '
                };
                buf.push(c);
            }
        }
        print!("\x1b[2J\x1b[0;0H{}", buf);
    }
}
