# check all rust crates
alias c := check
check:
    cargo check --workspace --tests


# build all rust crates
alias b := build
build:
    cargo build --workspace

# lint all rust crates
alias l := lint
lint:
    cargo clippy --fix --all --all-targets --all-features
    cargo fmt

alias r := run
run:
    cargo run

# GitHub Actions related
run_ci:
    act --container-architecture linux/amd64
show_ci:
    act --graph --container-architecture linux/amd64

clean:
    /bin/rm -rf target/*
