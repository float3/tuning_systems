#!/usr/bin/env bash

cargo update --workspace --verbose
cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features --workspace --verbose -- -D warnings
cargo fix --allow-dirty --allow-staged --all-targets --all-features --workspace --verbose
cargo fmt --all --verbose
cargo check --all-targets --all-features --workspace --verbose
cargo test --all-targets --all-features --workspace --verbose

cd ./tuningplayground
wasm-pack build --target web --dev

cd ../ts
npm update
npm audit fix
npx prettier . --write
npx eslint . --fix --ext .ts

cd ../music21-rs
./python_test.sh

cd ./music21
git pull origin master