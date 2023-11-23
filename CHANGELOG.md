# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changes

- Set the default loglevel for OnResponse to info.

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
