use rayon::prelude::*;
use std::{
    ffi::OsString,
    io::{self, Write},
};
use tree_sitter::{Language, Parser, Query, QueryCursor};
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

struct Type {
    name: String,
    line_number: usize,
}

struct TypeParser {
    language: Language,
    query: Query,
}

impl TypeParser {
    fn new() -> Self {
        let language = language();

        let query = Query::new(
            language,
            "[(class_declaration) (type_alias) (object_declaration)] @type",
        )
        .expect("query is invalid");

        return Self { language, query };
    }

    fn parse(&self, path: &OsString) -> Result<Vec<Type>, io::Error> {
        let mut parser = Parser::new();
        parser
            .set_language(self.language)
            .expect("language is generated with an incompatible version of tree-sitter");

        let mut cursor = QueryCursor::new();

        let source = std::fs::read_to_string(path)?;

        let tree = parser
            .parse(&source, None)
            .expect("pre-conditions aren't satisfied");

        let matches = cursor.captures(&self.query, tree.root_node(), |_| "");

        let types = matches
            .map(|(m, _)| m)
            .flat_map(|m| m.captures)
            .filter_map(|c| c.node.child_by_field_name("identifier"))
            .map(|id_node| Type {
                name: source[id_node.byte_range()].to_string(),
                line_number: id_node.start_position().row,
            })
            .collect();

        Ok(types)
    }
}
