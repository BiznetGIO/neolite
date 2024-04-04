#!/usr/bin/env -S just --justfile

set dotenv-load := true

alias d := dev
alias r := run
alias f := fmt
alias l := lint
alias t := test
alias c := comply
alias k := check

# List available commands.
_default:
    just --list --unsorted

# Setup the repository.
setup:
    # Please install: 'cargo-edit cargo-nextest cargo-outdated dprint git-cliff bacon typos-cli'

# Tasks to make the code-base comply with the rules. Mostly used in git hooks.
comply: _doc-check fmt lint test

# Check if the repository comply with the rules and ready to be pushed.
check: _doc-check fmt-check lint test

# Develop the app.
dev:
    bacon

# Run the app.
run:
    cargo run

# Format the codebase.
fmt:
    cargo fmt --all
    dprint fmt

# Check is the codebase properly formatted.
fmt-check:
    cargo fmt --all -- --check
    dprint check

# Lint the codebase.
lint:
    cargo clippy --all-targets --all-features
    typos --config configs/typos.toml

# Test the codebase.
test:
    cargo nextest run

# Run the unit tests.
test-unit:
    cargo nextest run --lib

# Create a new release. Example `cargo-release release minor --tag-name v0.2.0`
release level:
    cargo-release release {{ level }} --execute

# Make sure the repo is ready for release
release-check level: check
    just up
    cargo-release release {{ level }}

# Check the documentation.
_doc-check:
    cargo doc --all-features --no-deps

# Release hooks
_release-prepare version:
    git-cliff --config configs/cliff.toml --output CHANGELOG.md --tag {{ version }}
    just fmt

# Check dependencies health. Pass `--write` to uppgrade dependencies.
[unix]
up arg="":
    #!/usr/bin/env bash
    if [ "{{ arg }}" = "--write" ]; then
        cargo upgrade
        cargo update
    else
        cargo outdated --root-deps-only
    fi;

[windows]
up arg="":
    #!powershell.exe
    if ( "tool" -eq "--write") {
        cargo upgrade
        cargo update
    }
    else {
        cargo outdated --root-deps-only
    }
