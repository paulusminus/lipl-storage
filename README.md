[![Rust](https://github.com/paulusminus/lipl-storage/actions/workflows/rust.yml/badge.svg)](https://github.com/paulusminus/lipl-storage/actions/workflows/rust.yml)
[![Docker](https://github.com/paulusminus/lipl-storage/actions/workflows/docker.yml/badge.svg)](https://github.com/paulusminus/lipl-storage/actions/workflows/docker.yml)

# Lipl Storage

A collection of crates that can be used to create a binary executable that handles
the storage and retrieval of lyrics and playlists.
Configuration of the storage backend is done through environment variables.

## lipl-core

Models and LiplRepo trait.
The latter is used to hide implementation details for the backend.

## lipl-storage-fs

Storage on the filesystem.

### Configuration example

```bash
export LIPL_STORAGE_REPO_TYPE=fs
export LIPL_STORAGE_FS_DIR=/home/paul/lipl_data/
```

## lipl-storage-memory

Non persistent storage on internal memory.

### Configuration example

```bash
export LIPL_STORAGE_REPO_TYPE=memory
export LIPL_STORAGE_MEMORY_SAMPLE=true
```

## lipl-storage-postgres

Storage on a postgres db.

### Configuration example

```bash
export LIPL_STORAGE_REPO_TYPE=postgres
export LIPL_STORAGE_POSTGRES_CONNECTION="host=/var/run/postgresql dbname=lipl"
```

## lipl-storage-redis

Storage on a redis server.

### Example configuration

```bash
export LIPL_STORAGE_REPO_TYPE=redis
export LIPL_STORAGE_REDIS_CONNECTION=redis://127.0.0.1/
```

## lipl-sample-data

Sample data that can be used to play a demo or for testing.

## lipl-storage-server

The server component handles web requests.

