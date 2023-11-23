# Changelog

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.1] - 2023-11-23

### Added

- Graceful shutdown when receiving SIGINT or SIGTERM. The latter is important when running in a container.
- Set the default loglevel to info when running in a container.
