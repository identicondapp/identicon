#!/bin/bash
set -e

cargo test $1 -- --nocapture
