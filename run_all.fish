#!/usr/bin/env fish
cargo fmt --all -- --check; and \
cargo clippy --all-targets --all-features -- -Dwarnings; and \
cargo build-all-features; and \
cargo test-all-features; and \
cargo llvm-cov clean --workspace; and \
cargo llvm-cov -F unstable --no-report; and \
cargo llvm-cov report --lcov --output-path lcov.info; and \
cargo llvm-cov report --html; and \
cargo doc