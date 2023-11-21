# Lipl

A collection of crates that can be used to create a binary executable that handles the storage and retrieval of lyrics and playlists.

# lipl-axum-postgres

Storage backend for a axum web server

# lipl-core

Models and LiplRepo trait. The latter is used to hide implementation details for the backend.

# lipl-repo-fs

Storage and retrieval with the help of the filesystem.

# lipl-repo-memory

Non persistent storage and retrievel through internal memory. 

# lipl-repo-postgres

Storage and retrieval with the help of postgres client connecting to a postgres db.

# lipl-repo-redis

Storage and retrieval with the help of redis client connection to a redis server.

# lipl-sample-data

Sample data that can be used to play a demo.

# lipl-server-axum

Storage and retrievel with the help of a postgres client connected to a postgres db.
