#!/bin/bash

export LIPL_STORAGE_REPO_TYPE=turso
export LIPL_STORAGE_TURSO_DATABASE_PATH=lipl.sqlite

./target/x86_64-unknown-linux-musl/release/lipl-storage-server
