#!/usr/bin/env fish
cargo llvm-cov clean --workspace; and \
cargo llvm-cov -F unstable --no-report; and \
cargo llvm-cov report --lcov --output-path lcov.info; and \
cargo llvm-cov report --html