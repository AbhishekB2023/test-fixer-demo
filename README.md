# test-fixer-demo

A tiny demo project for testing the **rust-fixer** GitHub issue auto-fixer tool.

## Purpose

This is a deliberately minimal Rust library with one known bug:

- `add(a, b)` returns `a - b` instead of `a + b`, so two of its unit tests fail.

It exists so you can exercise the rust-fixer workflow (fetch → genfix → apply+test → push+PR) against a real repo you control.

## Running tests

```sh
cargo test
```

Expected: `add_works` and `add_handles_negatives` fail until the bug is fixed.