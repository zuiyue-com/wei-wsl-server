#!/bin/bash
cargo build --release
cp target/release/wei-wsl-server ./bin/
killall wei-wsl-server
rm -rf /usr/bin/wei-wsl-server
cp target/release/wei-wsl-server /usr/bin/    
cp target/release/wei-wsl-server ../wei-release/wsl/
