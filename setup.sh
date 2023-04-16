#!/bin/zsh

echo "Building binary"
cargo build --release

echo "Moving binary and configs.yml"
cp target/release/config ~/bin/config
cp configs.yml ~/bin/configs.yml
