#!/usr/bin/env bash

set -euxo pipefail

bench=${1:-args_10_structs}

export CARGO_INCREMENTAL=0

cargo clean

cargo build --features "$bench" --release -p benchmarks

cd benchmarks

cargo asm --features "$bench" --no-color "benchmarks::$bench::builder_bench" > builder.dbg.s || true
cargo asm --features "$bench" --no-color "benchmarks::$bench::regular_bench" > regular.dbg.s || true

# If vscode is present, show diff:
 if command -v code; then
    code --diff regular.dbg.s builder.dbg.s
fi

cargo bench --features "$bench" -p benchmarks --profile release --bench iai
cargo bench --features "$bench" -p benchmarks --profile release --bench criterion