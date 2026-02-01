#!/bin/bash

export RUST_LOG=info
export LIPL_STORAGE_REPO_TYPE=fs
export LIPL_STORAGE_FS_DIR="./data/"
export WWW_ROOT="/home/paul/Code/dart/lipl-control/packages/app/build/web"
./target/x86_64-unknown-linux-musl/release/lipl-storage-server
