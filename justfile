build:
    cargo build

build-release:
    cargo build --release

run:
    cargo run

run-release:
    cargo run --release

format:
    cargo fmt --check
    eclint -exclude "Cargo.lock" -exclude "flake.lock"

format-fix:
    cargo fmt
    eclint -exclude "Cargo.lock" -exclude "flake.lock" -fix

lint:
    cargo clippy

lint-fix:
    cargo clippy --fix

reuse:
    reuse lint
