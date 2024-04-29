#!/bin/bash

export RUST_LOG=info
export LIPL_STORAGE_REPO_TYPE=fs
export LIPL_STORAGE_FS_DIR="./out/"

./target/x86_64-unknown-linux-musl/release/lipl-storage-server
