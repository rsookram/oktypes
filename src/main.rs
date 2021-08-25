use std::io::Write;
use tree_sitter::{Language, Parser, Query, QueryCursor};

extern "C" {
    fn tree_sitter_kotlin() -> Language;
}

fn main() {
    // TODO: Add safety comment
    let language = unsafe { tree_sitter_kotlin() };

    let mut parser = Parser::new();
    // TODO: Handle errors
    parser.set_language(language).unwrap();

    // TODO: Query for typealiases too
    let query = Query::new(language, "(class_declaration) @class").unwrap();

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    // TODO: Parallelize
    for arg in std::env::args_os().skip(1) {
        let mut cursor = QueryCursor::new();

        let source = std::fs::read_to_string(&arg).unwrap();
        let tree = parser.parse(&source, None).unwrap();

        let matches = cursor.captures(&query, tree.root_node(), |_| "");

        for (m, _) in matches {
            let node = m.captures.first().unwrap().node;
            let id_node = node.child_by_field_name("identifier").unwrap();

            // TODO: ignore broken pipe errors
            writeln!(
                lock,
                "{}:{} {}",
                arg.to_string_lossy(),
                id_node.start_position().row,
                &source[id_node.byte_range()]
            )
            .unwrap();
        }
    }
}
