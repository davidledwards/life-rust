mod display;
mod error;
mod opt;
mod term;
mod universe;

use display::Display;
use opt::Options;
use random::Source;
use std::env;
use std::process::ExitCode;
use std::thread;
use std::time::{Duration, SystemTime};
use term::get_window_size;
use universe::{Point, Universe};

fn main() -> ExitCode {
    let opts = match Options::parse(env::args().skip(1)) {
        Err(e) => {
            println!("error: {:?}", e);
            return ExitCode::from(1);
        }
        Ok(opts) => opts,
    };
    match opts.help {
        None => {
            let win_size = match get_window_size() {
                Err(e) => {
                    eprintln!("error: {e}");
                    return ExitCode::from(1);
                }
                Ok(ws) => (ws.1 as u32, ws.0 as u32),
            };
            let xsize = opts.x.unwrap_or(win_size.0);
            let ysize = opts.y.unwrap_or(win_size.1);
            let bound = (xsize, ysize);
            let start = opts
                .start
                .or(Some(0))
                .map(|n| if n == 0 { xsize * ysize / 5 } else { n })
                .unwrap();
            let gens = opts.gens.unwrap_or(0);
            let delay = opts
                .delay
                .or(Some(500))
                .map(|n| Duration::from_millis(n))
                .unwrap();
            let fancy = opts.fancy.unwrap_or(false);

            let mut u = Universe::new(bound, random_genesis(start, bound));
            let disp = Display::new(bound);

            loop {
                if fancy {
                    disp.render_fancy(&u);
                } else {
                    disp.render_basic(&u);
                }
                thread::sleep(delay);
                if gens == 0 || u.gen < gens {
                    u = u.tick();
                } else {
                    break;
                }
            }
        }
        _ => print_usage(),
    }
    ExitCode::SUCCESS
}

fn print_usage() {
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

fn random_genesis(n: u32, bound: (u32, u32)) -> Vec<Point> {
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut rand = random::default(seed);
    let (xsize, ysize) = bound;
    (0..n)
        .map(|_| {
            let x = rand.read::<u32>() % xsize;
            let y = rand.read::<u32>() % ysize;
            Point(x as i32, y as i32)
        })
        .collect::<Vec<_>>()
}
