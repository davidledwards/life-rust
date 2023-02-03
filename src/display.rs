use crate::{Point, Universe};

pub trait Display {
    fn render(&self, u: &Universe);
}

pub fn basic(xsize: u32, ysize: u32) -> Box<dyn Display> {
    Box::new(BasicDisplay::new(xsize, ysize))
}

pub fn fancy(xsize: u32, ysize: u32) -> Box<dyn Display> {
    Box::new(FancyDisplay::new(xsize, ysize))
}

struct BasicDisplay {
    xsize: u32,
    ysize: u32,
}

struct FancyDisplay {
    xsize: u32,
    ysize: u32,
}

impl BasicDisplay {
    fn new(xsize: u32, ysize: u32) -> BasicDisplay {
        BasicDisplay { xsize, ysize }
    }
}

impl FancyDisplay {
    fn new(xsize: u32, ysize: u32) -> FancyDisplay {
        FancyDisplay { xsize, ysize }
    }
}

impl Display for BasicDisplay {
    fn render(&self, u: &Universe) {
        let mut buf = String::new();
        for y in 0..self.ysize as i32 {
            for x in 0..self.xsize as i32 {
                let s = if u.alive.contains(&Point(x, y)) {
                    "\x1b[0;;44m \x1b[0m"
                } else {
                    " "
                };
                buf.push_str(s);
            }
            buf.push('\n');
        }
        print!("\x1b[2J\x1b[0;0H{}", buf);
    }
}

impl Display for FancyDisplay {
    fn render(&self, u: &Universe) {
        let mut buf = String::new();
        for y in 0..self.ysize as i32 {
            for x in 0..self.xsize as i32 {
                let p = Point(x, y);
                let s = if u.born.contains(&p) {
                    "\x1b[0;;42m \x1b[0m"
                } else if u.alive.contains(&p) {
                    "\x1b[0;;44m \x1b[0m"
                } else if u.died.contains(&p) {
                    "\x1b[0;;41m \x1b[0m"
                } else {
                    " "
                };
                buf.push_str(s);
            }
            buf.push('\n');
        }
        print!("\x1b[2J\x1b[0;0H{}", buf);
    }
}
