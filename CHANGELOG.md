# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.7]

### Features

- pwa: Add ability to serve static files for [pwa](https://en.wikipedia.org/wiki/Progressive_web_app) functionallity


## [0.4.5]

### Changes

- Update dependencies.

## [0.4.4]

### Breaking Changes

- Accessing lyrics or playlists requires basic authentication. 
- Adding a health url that dos not require authentication.

## [0.4.3]

### Breaking Changes

- Changed the base path for the json api to /lipl/api/v1/

## [0.4.2]

### Breaking Changes

- Build one binary with all features enabled. Thus xtask build-bins is removed.

## [0.4.1]

- Include Cargo.lock for caching on Github Actions
- cargo update
- Docker image based on alpine 3.19

## Fix
- Clippy warning in package build-bins

## [0.4.0]

### Breaking Changes

- Rename the function to create a route. Move create_services to separate function.
- Axum version 0.7

## [0.3.5] - 2023-11-23

### Changes

- Set the working directory in the Dockerfile
- Use a xtask build-bins for building the 3 executables for fs, postgres and redis backend.

## [0.3.4] - 2023-11-23

### Changes

- Set the default loglevel for MakeSpan to info. Otherwise the details of the request are not printed.

## [0.3.3] - 2023-11-23

### Changes

- Set the default loglevel for OnResponse to info. This leeds to a simpler Quadlet definition.

## [0.3.2] - 2023-11-23

### Added

- Make the current directory default if LIPL_STORAGE_REPO_TYPE=fs and LIPL_STORAGE_FS_DIR is not provided.
- Add a directory quadlet with sample files for running containers with systemd.

### Fixed
- Documentation

## [0.3.1] - 2023-11-23

### Added

- Graceful shutdown when receiving SIGINT or SIGTERM. The latter is important when running in a container.
- Set the default loglevel to info when running in a container.
