mod display;
mod error;
mod opt;
mod term;
mod universe;

use crate::display::Display;
use crate::error::Error;
use crate::opt::Options;
use crate::universe::Universe;
use std::env;
use std::process::ExitCode;
use std::thread;
use std::time::Duration;

const USAGE: &str = r#"
usage: life OPTIONS
       life --help

  Runs a simulation of Conway's Game of Life on the terminal.

  The --x and --y options, if omitted, are derived from the terminal size.

  Living cells are displayed as blue blocks. If --fancy is specified, cells
  for each generation are displayed as follows:
    > green, if born
    > blue, if alive but born in prior generation
    > red, if died

  optional:
    --x SIZE         : width of universe (default is width of terminal)
    --y SIZE         : height of universe (default is height of terminal)
    --start COUNT    : number of living cells at start; 0 means derive
                       from universe size (default=0)
    --gen GENS       : number of generations to simulate; 0 is forever
                       (default=0)
    --delay MILLIS   : delay in milliseconds before next generation
                       (default=100)
    --fancy          : use fancy display (default is basic)
"#;

fn main() -> ExitCode {
    match run() {
        Err(e) => {
            match e {
                Error::Options(s) => {
                    println!("{}", s);
                    println!("use --help for options");
                }
                Error::IO(s) => println!("I/O error: {}", s),
            }
            ExitCode::from(1)
        }
        Ok(_) => ExitCode::SUCCESS,
    }
}

fn run() -> Result<(), Error> {
    let opts = Options::parse(env::args().skip(1))?;
    if opts.help {
        println!("{}", USAGE);
        Ok(())
    } else {
        simulate(opts)
    }
}

fn simulate(opts: Options) -> Result<(), Error> {
    let (term_xsize, term_ysize) = term::get_window_size()?;
    let xsize = opts.x.unwrap_or(term_xsize);
    let ysize = opts.y.unwrap_or(term_ysize - 1);
    let start = if opts.start == 0 {
        xsize * ysize / 5
    } else {
        opts.start
    };
    let gens = opts.gens;
    let delay = Duration::from_millis(opts.delay);
    let disp = if opts.fancy {
        Display::fancy(xsize, ysize)
    } else {
        Display::basic(xsize, ysize)
    };

    let mut u = Universe::new(xsize, ysize, universe::random_alive(start, xsize, ysize));
    loop {
        disp.render(&u);
        thread::sleep(delay);
        if gens == 0 || u.gen < gens {
            u = u.tick();
        } else {
            break;
        }
    }
    Ok(())
}
