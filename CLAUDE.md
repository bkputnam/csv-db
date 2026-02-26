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

## Architecture

The codebase is split into a library (`src/lib.rs`) and a thin binary (`src/main.rs`).

Core modules:
- `types` — `DataType` (INT/FLOAT/TEXT/BOOL) and `Value` (the atom of the database). `Value::matches_type` and `Value::is_null` are the key methods.
- `row` — `Row`: an ordered `Vec<Value>`. Implements `From<IntoIterator<Item=Value>>`.
- `schema` — `Column` and `Schema`. `Schema::validate_row` is the central validation entry point; it checks column count, type compatibility, and nullability.
- `error` — `DbError` enum and `Result<T>` alias used throughout.

Planned future modules: `table` (CSV I/O), `database` (multi-table management), `executor` (query execution against the `sqlparser` AST).

## Notes

- Uses Rust edition 2024 — prefer modern Rust idioms.
- The `.gitignore` excludes `mutants.out*/` (cargo-mutants output).
