#!/bin/bash

export LIPL_STORAGE_REPO_TYPE=postgres
export LIPL_STORAGE_POSTGRES_CONNECTION="host=/run/postgresql/ dbname=test2"

./target/x86_64-unknown-linux-musl/release/lipl-storage-postgres
