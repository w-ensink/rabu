pre-commit: check-fmt build test

test:
    cargo test --verbose

build:
    cargo build

fmt:
    cargo fmt

check-fmt:
    cargo fmt --check