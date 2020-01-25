#!/bin/bash

set -xe

cargo clean --doc

cargo fmt --all -- --check
cargo test --release
cargo test --release --all-features
cargo bench --no-run

cargo doc --all-features
linkchecker target/doc/linuxcnc_hal_sys/index.html
linkchecker target/doc/linuxcnc_hal/index.html

# Check that sys package builds into something pushable to crates.io
pushd linuxcnc-hal-sys
cargo package
popd
