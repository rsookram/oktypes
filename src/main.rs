use rayon::prelude::*;
use std::{
    ffi::OsString,
    io::{self, Write},
};
use tree_sitter::{Parser, Query, QueryCursor};
use tree_sitter_kotlin::language;

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
    let language = language();

    let query = Query::new(
        language,
        "[(class_declaration) (type_alias) (object_declaration)] @type",
    )
    .expect("query is invalid");

    let stdout = io::stdout();

    args.par_iter().try_for_each(|arg| {
        let mut parser = Parser::new();
        parser
            .set_language(language)
            .expect("language is generated with an incompatible version of tree-sitter");

        let mut cursor = QueryCursor::new();

        let source = match std::fs::read_to_string(&arg) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("{}: {}", arg.to_string_lossy(), err);
                return Err(err);
            }
        };

        let tree = parser
            .parse(&source, None)
            .expect("pre-conditions aren't satisfied");

        let matches = cursor.captures(&query, tree.root_node(), |_| "");

        let mut lock = stdout.lock();

        let result = matches
            .map(|(m, _)| m)
            .flat_map(|m| m.captures)
            .filter_map(|c| c.node.child_by_field_name("identifier"))
            .try_for_each(|id_node| {
                writeln!(
                    lock,
                    "{}:{} {}",
                    arg.to_string_lossy(),
                    id_node.start_position().row,
                    &source[id_node.byte_range()]
                )
            });

        result
    })
}
