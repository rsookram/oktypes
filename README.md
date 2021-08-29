# oktypes

oktypes is a command line tool that outputs the names of types defined in Kotlin source files.


## Install

Currently, pre-compiled binaries of oktypes aren't being distributed. You can install it with
[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) by running

```
cargo install --git https://github.com/rsookram/oktypes
```


## Usage

Simply call `oktypes` from the command line and pass the paths of the Kotlin files to look at as
arguments.

For example, if you want to see all the types defined in `*.kt` files in a git repository, you can
run:

```shell
git ls-files '*.kt' | xargs oktypes
```


## Build

oktypes can be built from source by cloning this repository and using Cargo.

```
git clone https://github.com/rsookram/oktypes
cd calr
cargo build --release
```


## Dependencies

The following dependencies are used to implement oktypes:

- [`tree-sitter`](https://crates.io/crates/tree-sitter) to generate a parser for parsing Kotlin
source code.
- [`tree-sitter-kotlin`](https://github.com/rsookram/tree-sitter-kotlin) (my fork) to define the
tree-sitter grammar
- [`rayon`](https://crates.io/crates/rayon) to process input files in parallel
