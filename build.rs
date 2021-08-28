fn main() -> Result<(), cc::Error> {
    // Compile tree-sitter grammar
    cc::Build::new()
        .include("tree-sitter-kotlin/src")
        .file("tree-sitter-kotlin/src/parser.c")
        .try_compile("kotlin")?;

    // Only rebuild if the code generated for the grammar changed
    println!("cargo:rerun-if-changed=tree-sitter-kotlin/src");

    Ok(())
}
