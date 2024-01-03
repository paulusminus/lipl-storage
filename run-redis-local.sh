#!/bin/bash

export LIPL_STORAGE_REPO_TYPE=redis
export LIPL_STORAGE_REDIS_CONNECTION=redis://127.0.0.1/

./target/x86_64-unknown-linux-musl/release/lipl-storage-server
