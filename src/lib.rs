use std::ffi::OsString;
use tree_sitter::{Language, Parser, Query, QueryCursor};
use tree_sitter_kotlin::language;

pub struct Type {
    pub name: String,
    pub line_number: usize,
}

pub struct TypeParser {
    language: Language,
    query: Query,
}

impl TypeParser {
    pub fn new() -> Self {
        let language = language();

        let query = Query::new(
            language,
            "[(class_declaration) (type_alias) (object_declaration)] @type",
        )
        .expect("query is invalid");

        return Self { language, query };
    }

    // TODO: Make the caller read the data instead
    pub fn parse(&self, path: &OsString) -> Result<Vec<Type>, std::io::Error> {
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
