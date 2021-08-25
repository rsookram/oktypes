fn main() -> Result<(), cc::Error> {
    // Compile tree-sitter grammar
    cc::Build::new()
        .include("tree-sitter-kotlin/src")
        .file("tree-sitter-kotlin/src/parser.c")
        .try_compile("kotlin")?;

    // TODO: emit rerun-if-changed

    Ok(())
}
