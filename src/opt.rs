use crate::error::Error;

#[derive(Debug)]
pub struct Options {
    pub help: Option<bool>,
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub start: Option<u32>,
    pub gens: Option<u32>,
    pub delay: Option<u64>,
    pub fancy: Option<bool>,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            help: None,
            x: None,
            y: None,
            start: None,
            gens: None,
            delay: None,
            fancy: None,
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
                "-?" | "--help" => opts.help = Some(true),
                "-x" => opts.x = Some(parse_arg(&arg, it.next())?),
                "-y" => opts.y = Some(parse_arg(&arg, it.next())?),
                "--start" => opts.start = Some(parse_arg(&arg, it.next())?),
                "--gens" => opts.gens = Some(parse_arg(&arg, it.next())?),
                "--delay" => opts.delay = Some(parse_arg(&arg, it.next())? as u64),
                "--fancy" => opts.fancy = Some(true),
                _ => return Err(Error::Options(format!("{}: unexpected argument", arg))),
            };
        }
        Ok(opts)
    }
}

fn parse_arg(arg: &String, next: Option<String>) -> Result<u32, Error> {
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
