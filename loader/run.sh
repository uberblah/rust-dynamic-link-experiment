#!/bin/sh

pushd ../a > /dev/null
cargo build
cp target/debug/liba.dylib ../loader
popd > /dev/null

cargo run
