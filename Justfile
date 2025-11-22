download:
    cargo run --manifest-path download-tree-sitter/Cargo.toml

test:
    cargo run -- compile --format pdf tree-sitter.typ
    firefox tree-sitter.pdf

build:
    cargo build --release
    mv target/release/typst.exe ~/.cargo/bin
