mod display;
mod error;
mod opt;
mod term;
mod universe;

use error::Error;
use opt::Options;
use std::env;
use std::process::ExitCode;
use std::thread;
use std::time::Duration;
use universe::{Point, Universe};

fn main() -> ExitCode {
    match run() {
        Err(e) => {
            match e {
                Error::Options(s) => println!("{}", s),
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
        usage();
        Ok(())
    } else {
        simulate(opts)
    }
}

fn simulate(opts: Options) -> Result<(), Error> {
    let win_size = term::get_window_size()?;
    let xsize = opts.x.unwrap_or(win_size.1 as u32);
    let ysize = opts.y.unwrap_or(win_size.0 as u32);
    let start = if opts.start == 0 {
        xsize * ysize / 5
    } else {
        opts.start
    };
    let gens = opts.gens;
    let delay = Duration::from_millis(opts.delay);
    let disp = if opts.fancy {
        display::fancy(xsize, ysize)
    } else {
        display::basic(xsize, ysize)
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

fn usage() {
    println!("usage: life OPTIONS");
    println!("       life -? | --help");
    println!();
    println!("  Runs a simulation of Conway's Game of Life on the console.");
    println!();
    println!("  The -x and -y options, if omitted, are derived from the terminal size.");
    println!();
    println!("  If --fancy is specified, cells for each generation are displayed as follows:");
    println!("    `+` if born");
    println!("    `#` if alive, but born in prior generation");
    println!("    `-` if died");
    println!();
    println!("optional:");
    println!("  -x SIZE          : width of universe (default is width of terminal");
    println!("  -y SIZE          : height of universe (default is height of terminal");
    println!("  --start COUNT    : number of living cells at start; 0 means derive");
    println!("                     derive from universe size (default=0)");
    println!("  --gen GENS       : number of generations to simulate; 0 is forever");
    println!("                     (default=0)");
    println!("  --delay MILLIS   : delay in milliseconds before next generation");
    println!("                     (default=500)");
    println!("  --fancy          : use fancy display (default is basic)");
}
