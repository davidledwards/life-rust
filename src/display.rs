use crate::universe::{Point, Universe};

pub struct Display {
    xsize: u32,
    ysize: u32,
    render_point: fn(&Point, &Universe) -> &'static str,
}

impl Display {
    pub fn basic(xsize: u32, ysize: u32) -> Display {
        Display {
            xsize,
            ysize,
            render_point: Display::render_basic,
        }
    }

    pub fn fancy(xsize: u32, ysize: u32) -> Display {
        Display {
            xsize,
            ysize,
            render_point: Display::render_fancy,
        }
    }

    pub fn render(&self, u: &Universe) {
        if u.gen == 0 {
            println!("\x1b[2J");
        }
        let mut buf = String::new();
        for y in 0..self.ysize as i32 {
            for x in 0..self.xsize as i32 {
                let p = Point(x, y);
                buf.push_str((self.render_point)(&p, &u));
            }
            buf.push('\n');
        }
        print!("\x1b[H{}", buf);
    }

    fn render_basic(p: &Point, u: &Universe) -> &'static str {
        if u.alive.contains(&p) {
            "\x1b[0;;44m \x1b[0m"
        } else {
            " "
        }
    }

    fn render_fancy(p: &Point, u: &Universe) -> &'static str {
        if u.born.contains(&p) {
            "\x1b[0;;42m \x1b[0m"
        } else if u.alive.contains(&p) {
            "\x1b[0;;44m \x1b[0m"
        } else if u.died.contains(&p) {
            "\x1b[0;;41m \x1b[0m"
        } else {
            " "
        }
    }
}
