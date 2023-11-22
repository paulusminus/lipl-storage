#!/bin/bash

export LIPL_STORAGE_REPO_TYPE=fs
export LIPL_STORAGE_FS_DIR="./data_lipl/"

./target/x86_64-unknown-linux-musl/release/lipl-storage-fs
