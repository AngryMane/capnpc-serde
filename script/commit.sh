#/usr/bin/env bash
cd capnpc-serde
cargo fmt
cargo clippy
cargo doc --no-deps
rm -rf ../doc
cp -r target/doc ../