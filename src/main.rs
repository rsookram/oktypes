use rayon::prelude::*;
use std::io::{self, Write};
use tree_sitter::{Language, Parser, Query, QueryCursor};

extern "C" {
    fn tree_sitter_kotlin() -> Language;
}

fn main() {
    // TODO: Add safety comment
    let language = unsafe { tree_sitter_kotlin() };

    // TODO: Query for typealiases too
    let query = Query::new(language, "(class_declaration) @class").expect("query is invalid");

    let stdout = io::stdout();

    let args = std::env::args_os().skip(1).collect::<Vec<_>>();

    let result = args.par_iter().try_for_each(|arg| {
        let mut parser = Parser::new();
        parser
            .set_language(language)
            .expect("language is generated with an incompatible version of tree-sitter");

        let mut cursor = QueryCursor::new();

        let source = std::fs::read_to_string(&arg)?;
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
    });

    if let Err(err) = result {
        if err.kind() == io::ErrorKind::BrokenPipe {
            // Ignore broken pipe errors to better handle usage within a pipeline (e.g.
            // `oktypes ...  | head -n1`)
        } else {
            eprintln!("{}: {}", "TODO: include filename in error", err);
            std::process::exit(err.raw_os_error().unwrap_or(1));
        }
    }
}
