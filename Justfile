#!/usr/bin/env just --justfile

release:
  cargo build --release    

lint:
  cargo clippy

bin:
  cargo run -- -t "Hello, World!" -o hello_world.png -l