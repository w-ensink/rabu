pre-commit: check-fmt assert-no-warnings build test

test:
    cargo test --verbose

build:
    cargo build

fmt:
    cargo fmt

check-fmt:
    cargo fmt --check

assert-no-warnings:
    cargo clippy -- -D warnings