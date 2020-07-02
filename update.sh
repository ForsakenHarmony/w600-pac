#!/usr/bin/env bash

# Before running this script, install the required software:
# cargo install svd2rust --version=0.16.1
# cargo install form --version=0.7.0
# pip3 install --upgrade --user svdtools

set -x
set -e

rm -rf src
mkdir src
svd patch patches/w600.yaml
svd2rust -i W600.svd.patched
form -i lib.rs -o src
rm lib.rs
cargo fmt
