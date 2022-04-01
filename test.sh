#!/bin/bash
set -e

RUSTFLAGS='-A dead_code -A unused_variables -C link-arg=-s' cargo test $1 -- --nocapture
