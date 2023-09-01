#!/bin/sh
cargo run --package adder & \
cargo run --package subtractor & \
cargo run --package multiplier & \
cargo run --package divider
