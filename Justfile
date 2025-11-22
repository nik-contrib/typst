tree-sitter-download:
    cargo run --manifest-path download-tree-sitter/Cargo.toml

tree-sitter-test:
    cargo run -- compile --format pdf tree-sitter.typ
    firefox tree-sitter.pdf
