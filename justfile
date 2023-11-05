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
    eclint -exclude "{Cargo.lock,flake.lock,**/*.javap}"

format-fix:
    cargo fmt
    eclint -exclude "{Cargo.lock,flake.lock,**/*.javap}" -fix

lint:
    cargo clippy

lint-fix:
    cargo clippy --fix

reuse:
    reuse lint

compile javaSource:
    javac -g -source 8 -target 8 "{{ javaSource }}"

javap javaClass:
    javap -verbose -l -private -s -sysinfo -constants "{{ javaClass }}"
