# justfile 

dev:
    cargo watch -c -w src -x run

build:
    cargo build --release