#!/usr/bin/env bash

cargo fmt --all
cargo clippy --fix --allow-dirty
