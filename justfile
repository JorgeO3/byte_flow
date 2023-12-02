#!/usr/bin/env just --justfile

release:
  cargo build --release    

lint:
  cargo clippy

bin:
  cargo run --bin client -- arg1

example:
  cargo run --example exname -- arg1