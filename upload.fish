#!/usr/bin/env fish
cargo build --release --target=x86_64-unknown-linux-gnu
rm -rf build
mkdir -p build
cp -r target/x86_64-unknown-linux-gnu/release/personal-website build/
cp -r static/ build/
gcloud compute scp --recurse build/* personal-website-server:~
