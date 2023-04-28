use crate::error::Error;

#[derive(Debug)]
pub struct Options {
    pub help: bool,
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub start: u32,
    pub gens: u32,
    pub delay: u64,
    pub fancy: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            help: false,
            x: None,
            y: None,
            start: 0,
            gens: 0,
            delay: 100,
            fancy: false,
        }
    }
}

impl Options {
    pub fn parse<T>(args: T) -> Result<Options, Error>
    where
        T: IntoIterator<Item = String>,
    {
        let mut opts = Options::default();
        let mut it = args.into_iter();
        while let Some(arg) = it.next() {
            match arg.as_str() {
                "--help" => opts.help = true,
                "--x" => opts.x = Some(parse_arg(&arg, it.next())?),
                "--y" => opts.y = Some(parse_arg(&arg, it.next())?),
                "--start" => opts.start = parse_arg(&arg, it.next())?,
                "--gens" => opts.gens = parse_arg(&arg, it.next())?,
                "--delay" => opts.delay = parse_arg(&arg, it.next())? as u64,
                "--fancy" => opts.fancy = true,
                _ => return Err(Error::Options(format!("{}: unexpected argument", arg))),
            };
        }
        Ok(opts)
    }
}

fn parse_arg(arg: &str, next: Option<String>) -> Result<u32, Error> {
    match next {
        Some(a) => match a.parse::<u32>() {
            Ok(x) => Ok(x),
            Err(_) => Err(Error::Options(format!(
                "{}: invalid argument following {}",
                a, arg
            ))),
        },
        None => Err(Error::Options(format!(
            "{}: expecting argument to follow",
            arg
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opt_defaults() {
        let opts = Options::parse(Vec::new());
        assert!(opts.is_ok());
        let opts = opts.unwrap();
        assert_eq!(opts.help, false);
        assert_eq!(opts.x, None);
        assert_eq!(opts.y, None);
        assert_eq!(opts.start, 0);
        assert_eq!(opts.gens, 0);
        assert_eq!(opts.delay, 100);
        assert_eq!(opts.fancy, false);
    }

    #[test]
    fn opt_help() {
        let args = vec!["--help".to_string()];
        verify_opts(&args, |opts| opts.help == true);
    }

    #[test]
    fn opt_x() {
        let args = vec!["--x".to_string(), "7".to_string()];
        verify_opts(&args, |opts| opts.x == Some(7));
    }

    #[test]
    fn opt_y() {
        let args = vec!["--y".to_string(), "9".to_string()];
        verify_opts(&args, |opts| opts.y == Some(9));
    }

    #[test]
    fn opt_start() {
        let args = vec!["--start".to_string(), "123".to_string()];
        verify_opts(&args, |opts| opts.start == 123);
    }

    #[test]
    fn opt_gens() {
        let args = vec!["--gens".to_string(), "33".to_string()];
        verify_opts(&args, |opts| opts.gens == 33);
    }

    #[test]
    fn opt_delay() {
        let args = vec!["--delay".to_string(), "999".to_string()];
        verify_opts(&args, |opts| opts.delay == 999);
    }

    #[test]
    fn opt_fancy() {
        let args = vec!["--fancy".to_string()];
        verify_opts(&args, |opts| opts.fancy == true);
    }

    #[test]
    fn good_arg() {
        match parse_arg("--arg", Some("47".to_string())) {
            Ok(v) => assert_eq!(v, 47),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn bad_arg() {
        match parse_arg("--arg", Some("47_".to_string())) {
            Err(Error::Options(e)) => assert!(
                e.contains("47_: invalid argument following --arg"),
                "error: {}",
                e
            ),
            _ => panic!(),
        }
    }

    #[test]
    fn missing_arg() {
        match parse_arg("--arg", None) {
            Err(Error::Options(e)) => assert!(
                e.contains("--arg: expecting argument to follow"),
                "error: {}",
                e
            ),
            _ => panic!(),
        }
    }

    #[test]
    fn unsupported_opt() {
        let args = vec!["--help".to_string(), "--foo".to_string()];
        match Options::parse(args.clone()) {
            Err(Error::Options(e)) => assert!(
                e.contains("--foo: unexpected argument"),
                "error: {}, args: {:?}",
                e,
                args
            ),
            _ => panic!(),
        }
    }

    fn verify_opts<F>(args: &Vec<String>, test_fn: F)
    where
        F: Fn(&Options) -> bool,
    {
        match Options::parse(args.clone()) {
            Ok(opts) => assert!(test_fn(&opts), "args: {:?}", args),
            Err(e) => panic!("error: {:?}, args: {:?}", e, args),
        }
    }
}
