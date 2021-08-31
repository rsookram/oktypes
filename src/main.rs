use oktypes::TypeParser;
use rayon::prelude::*;
use std::{
    ffi::OsString,
    io::{self, Write},
};

fn main() {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();

    let result = run(&args);

    if let Err(err) = result {
        if err.kind() == io::ErrorKind::BrokenPipe {
            // Ignore broken pipe errors to better handle usage within a pipeline (e.g.
            // `oktypes ...  | head -n1`)
        } else {
            std::process::exit(err.raw_os_error().unwrap_or(1));
        }
    }
}

fn run(args: &[OsString]) -> Result<(), io::Error> {
    let parser = TypeParser::new();

    let stdout = io::stdout();

    args.par_iter().try_for_each(|arg| {
        let types = match parser.parse(arg) {
            Ok(t) => t,
            Err(err) => {
                eprintln!("{}: {}", arg.to_string_lossy(), err);
                return Err(err);
            }
        };

        let mut lock = stdout.lock();

        types.iter().try_for_each(|t| {
            writeln!(
                lock,
                "{}:{} {}",
                arg.to_string_lossy(),
                t.line_number,
                t.name,
            )
        })
    })
}
