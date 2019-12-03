#!/bin/bash

cat $1/input/input.txt | cargo run --manifest-path $1/Cargo.toml
