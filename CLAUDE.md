# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`csv-db` is a Rust project (edition 2024) intended to be a database that operates on CSV files. It is in early development.

## Commands

```bash
# Build
cargo build

# Run
cargo run

# Run tests
cargo test

# Run a single test
cargo test <test_name>

# Check for errors without building
cargo check

# Format code
cargo fmt

# Lint
cargo clippy

# Mutation testing (cargo-mutants)
cargo mutants
```

## Notes

- Uses Rust edition 2024 — prefer modern Rust idioms.
- No external dependencies yet; add them to `Cargo.toml` as needed.
- The `.gitignore` excludes `mutants.out*/` (cargo-mutants output).
